//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1170/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1170(t2282: f64, t2311: f64, t6666: f64, t835: f64, t20730: f64, t275: f64, t2289: f64, t6640: f64, t20741: f64, t2243: f64, t2272: f64, t6709: f64, t816: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20990 = t2282 * t2311;
    let t20995 = t835 * t6666;
    let t21000 = t275 * t20730;
    let t21004 = t2282 * t2289;
    let t21007 = t835 * t6640;
    let t21037 = t275 * t20741;
    let t21043 = t2243 * t2272;
    let t21048 = t816 * t6709;
    (t20990, t20995, t21000, t21004, t21007, t21037, t21043, t21048)
}
