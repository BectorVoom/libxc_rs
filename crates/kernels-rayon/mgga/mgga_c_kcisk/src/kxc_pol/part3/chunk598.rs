//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 598/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk598(t1802: f64, t4581: f64, t1799: f64, t1333: f64, t1865: f64, t568: f64, t967: f64, t682: f64, t1810: f64, t696: f64, t1806: f64, t1825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5077 = t4581 * t1802;
    let t5078 = t1799 * t5077;
    let t5080 = t1333 * t1865;
    let t5082 = t967 * t568;
    let t5084 = 0.46853067927761790996e-2_f64 * t5082 * t682;
    let t5085 = t696 * t1810;
    let t5087 = t1806 * t1825;
    (t5077, t5078, t5080, t5082, t5084, t5085, t5087)
}
