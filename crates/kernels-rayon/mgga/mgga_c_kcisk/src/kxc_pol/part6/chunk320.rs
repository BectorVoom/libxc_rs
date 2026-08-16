//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 320/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk320(t695: f64, t967: f64, t227: f64, t694: f64) -> (f64, f64, f64, f64) {
    let t1846 = t967 * t695;
    let t1847 = 0.5179538907796306876e-4_f64 * t1846;
    let t1848 = t694 * t227;
    let t1849 = 1.0_f64 / t1848;
    (t1846, t1847, t1848, t1849)
}
