//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1320/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1320(t118830: f64, t1484: f64, t865: f64, t22986: f64, t23270: f64, t30633: f64, t112867: f64, t1880: f64, t23237: f64, t32866: f64, t1888: f64, t4300: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t118831 = 0.82246703342411321825e-2_f64 * t118830;
    let t118833 = t1484 * t865;
    let t118837 = 0.6579736267392905746e-1_f64 * t22986 * t23270 * t30633 * t118833;
    let t118838 = 0.16449340668482264365e-1_f64 * t112867;
    let t118841 = 0.16449340668482264365e-1_f64 * t1880 * t23237 * t32866;
    let t118847 = 0.3289868133696452873e-1_f64 * t1888 * t23270 * t30633 * t4300;
    (t118831, t118833, t118837, t118838, t118841, t118847)
}
