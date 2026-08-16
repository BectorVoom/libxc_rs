//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2720/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2720(t1396: f64, t1398: f64, t1404: f64, t1852: f64, t1858: f64, t20149: f64, t20186: f64, t22431: f64, t22453: f64, t3: f64, t5364: f64, t5381: f64, t580: f64, t6471: f64, t6483: f64, t66964: f64, t66967: f64, t66976: f64, t66987: f64, t66989: f64, t66991: f64, t67000: f64, t75764: f64, t75768: f64, t75774: f64, t75780: f64, t75827: f64) -> f64 {
    let tv4rho43 = t3 * t580 * t75764 + t1396 * t22453 + t1398 * t75827 + t1404 * t22431 + 3.0_f64 * t1852 * t20186 + 3.0_f64 * t1858 * t20149 + 3.0_f64 * t5364 * t6483 + 3.0_f64 * t5381 * t6471 + 6.0_f64 * t66964 + 3.0_f64 * t66967 + 3.0_f64 * t66976 + 3.0_f64 * t66987 + 3.0_f64 * t66989 + 6.0_f64 * t66991 + t67000 + 3.0_f64 * t75768 + 3.0_f64 * t75774 + t75780;
    tv4rho43
}
