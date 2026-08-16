//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 602/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk602(t1409: f64, t3242: f64, t607: f64, t3240: f64, t123: f64, t3247: f64, t1088: f64, t1089: f64, t3966: f64, t3237: f64, t3238: f64, t4721: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4723 = t3242 * t1409;
    let t4724 = t4723 * t607;
    let t4725 = t3240 * t4724;
    let t4726 = t123 * t4725;
    let t4728 = t3247 * t1409;
    let t4729 = t4728 * t607;
    let t4730 = t1088 * t4729;
    let t4731 = t123 * t4730;
    let t4733 = t1089 * t3966;
    let t4734 = t1088 * t4733;
    let t4735 = t123 * t4734;
    let t4737 = t3237 - 0.5936111111111111111e-2_f64 * t3238 - 0.5936111111111111111e-2_f64 * t4721 - 0.11872222222222222222e-1_f64 * t4726 + 0.35616666666666666666e-1_f64 * t4731 + 0.17808333333333333333e-1_f64 * t4735;
    (t4724, t4726, t4729, t4731, t4733, t4735, t4737)
}
