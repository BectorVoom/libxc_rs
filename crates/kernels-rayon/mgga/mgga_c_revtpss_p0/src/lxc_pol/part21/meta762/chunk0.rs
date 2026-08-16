//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2702/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2702(t116: f64, t13424: f64, t2371: f64, t648: f64, t10199: f64, t1514: f64, t2289: f64, t4264: f64, t13459: f64, t625: f64, t13462: f64, t13510: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t49686 = t13424 * t116;
    let t49693 = t648 * t2371;
    let t49698 = t10199 * t1514;
    let t49700 = t2289 * t4264;
    let t49701 = 22.0_f64 / 3.0_f64 * t49700;
    let t49702 = t625 * t13459;
    let t49704 = t625 * t13462;
    let t49724 = t625 * t13510;
    (t49686, t49693, t49698, t49701, t49702, t49704, t49724)
}
