//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 902/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk902(t195: f64, t6750: f64, t2531: f64, t642: f64, t1821: f64, t998: f64, t1062: f64, t1469: f64, t2724: f64, t462: f64, t1020: f64, t568: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6751 = t6750 * t195;
    let t6752 = t2531 * t642;
    let t6754 = t998 * t1821;
    let t6755 = t1469 * t1062;
    let t6756 = t462 * t2724;
    let t6758 = t1020 * t568;
    (t6751, t6752, t6754, t6755, t6756, t6758)
}
