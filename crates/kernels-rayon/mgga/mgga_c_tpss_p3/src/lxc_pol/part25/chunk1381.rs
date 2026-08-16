//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1381/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1381(t1848: f64, t5480: f64, t1856: f64, t5465: f64, t1665: f64, t6458: f64, t21947: f64, t550: f64, t21984: f64, t546: f64, t1673: f64, t20649: f64, t3: f64, t4544: f64, t67858: f64, t67860: f64, t67868: f64, t67874: f64, t67879: f64, t72724: f64) -> f64 {
    let t72750 = t1848 * t5480;
    let t72751 = t5465 * t1856;
    let t72752 = t1665 * t6458;
    let t72754 = t21947 * t550;
    let t72755 = t546 * t21984;
    let t72756 = t3 * t550 * t72724 + 2.0_f64 * t1673 * t20649 + 2.0_f64 * t4544 * t6458 + t67858 + t67860 + t67868 + t67874 + t67879 + t72750 + t72751 + 2.0_f64 * t72752 + t72754 + t72755;
    t72756
}
