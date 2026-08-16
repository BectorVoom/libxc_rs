//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 456/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk456(t1017: f64, t67: f64, t3067: f64, t1058: f64, t1044: f64, t820: f64) -> (f64, f64, f64, f64) {
    let t3068 = t1017 * t67;
    let t3069 = t3067 * t3068;
    let t3070 = t1058 * t3069;
    let t3071 = t820 * t1044;
    (t3068, t3069, t3070, t3071)
}
