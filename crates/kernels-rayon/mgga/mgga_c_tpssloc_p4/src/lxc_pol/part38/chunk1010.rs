//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1010/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1010(t12300: f64, t1354: f64, t12189: f64, t1329: f64, t3726: f64, t3770: f64, t12211: f64, t3766: f64, t1358: f64, t3774: f64, t1333: f64, t3862: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12301 = t12300 * t1354;
    let t12308 = t12189 * t1329;
    let t12310 = t3726 * t3770;
    let t12317 = t12211 * t3766;
    let t12323 = t3774 * t1358;
    let t12325 = t1333 * t3862;
    (t12301, t12308, t12310, t12317, t12323, t12325)
}
