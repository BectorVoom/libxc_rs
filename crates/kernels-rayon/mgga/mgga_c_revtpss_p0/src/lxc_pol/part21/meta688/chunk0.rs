//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2507/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2507(t12984: f64, t3667: f64, t12976: f64, t3678: f64, t12963: f64, t1235: f64, t127: f64, t12970: f64, t371: f64, t126: f64, t13099: f64, t12257: f64, t1261: f64, t247: f64) -> (f64, f64, f64, f64, f64) {
    let t44884 = t3667 * t12984;
    let t44886 = t12976 * t3678;
    let t44888 = t3667 * t12963;
    let t44892 = t1235 * t371 * t127 * t12970;
    let t44895 = t126 * t13099;
    let t44898 = t1261 * t247 * t44895 * t12257;
    (t44884, t44886, t44888, t44892, t44898)
}
