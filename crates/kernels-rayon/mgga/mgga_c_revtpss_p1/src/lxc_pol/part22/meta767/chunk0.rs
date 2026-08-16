//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2849/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2849(t3494: f64, t3519: f64, t13026: f64, t240: f64, t3361: f64, t1146: f64, t9303: f64, t2304: f64, t2439: f64, t3424: f64, t3421: f64, t25273: f64, t268: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t43752 = 1.0_f64 / t3519 / t3494;
    let t43764 = t240 * t13026;
    let t43765 = t3361 * t3361;
    let t43766 = 1.0_f64 / t43765;
    let t43771 = t9303 * t1146;
    let t43776 = 1.0_f64 / t3361 / t2304;
    let t43781 = t2439 * t3424;
    let t43783 = t2439 * t3421;
    let t43813 = t268 * t25273 * t404;
    (t43752, t43764, t43766, t43771, t43776, t43781, t43783, t43813)
}
