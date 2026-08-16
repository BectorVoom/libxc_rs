//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 796/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk796(t4787: f64, t8607: f64, t4744: f64, t8573: f64, t1644: f64, t8544: f64, t682: f64, t8522: f64, t8504: f64, t1417: f64, t8928: f64, t719: f64, t8831: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t22760 = t4787 * t8607;
    let t22801 = t8573 * t4744;
    let t22891 = t8544 * t1644;
    let t22927 = t682 * t8522;
    let t22937 = t682 * t8504;
    let t22942 = t1417 * t8928;
    let t23033 = t8831 * t719;
    (t22760, t22801, t22891, t22927, t22937, t22942, t23033)
}
