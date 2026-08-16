//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 643/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk643(t1007: f64, t34: f64, t115: f64, t121: f64, t859: f64, t343: f64, t3818: f64, t107: f64, t837: f64) -> (f64, f64, f64, f64) {
    let t25561 = 1.0_f64 / t34 / t1007;
    let t25607 = t121 / t859 / t115;
    let t25636 = t343 * t3818;
    let t25640 = t107 * t837;
    (t25561, t25607, t25636, t25640)
}
