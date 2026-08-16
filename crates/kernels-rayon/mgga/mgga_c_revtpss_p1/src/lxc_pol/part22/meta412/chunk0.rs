//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2015/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2015(t3920: f64, t5603: f64, t2435: f64, t5718: f64, t1893: f64, t2453: f64, t3908: f64, t1904: f64, t3895: f64, t2439: f64, t213: f64, t5710: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t14280 = t5603 * t3920;
    let t14290 = t2435 * t5718;
    let t14293 = t2453 * t1893;
    let t14294 = t14293 * t3908;
    let t14296 = t3895 * t1904;
    let t14297 = t2439 * t14296;
    let t14299 = t213 * t5710;
    (t14280, t14290, t14293, t14294, t14296, t14297, t14299)
}
