//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 686/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk686(t1906: f64, t724: f64, t11225: f64, t732: f64, t640: f64, t719: f64, t10487: f64, t702: f64, t140: f64, t446: f64, t728: f64, t10459: f64, t41: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11699 = t1906 * t1906;
    let t11700 = 1.0_f64 / t11699;
    let t11701 = t724 * t11700;
    let t11774 = t732 * t11225;
    let t11775 = t11774 * sigma2;
    let t11807 = 1.0_f64 / t719 / t640;
    let t11832 = t702 * t10487;
    let t11885 = 0.11791604938271604938e-1_f64 * t140 * t446 * t728;
    let t11910 = t41 * t10459;
    (t11701, t11775, t11807, t11832, t11885, t11910)
}
