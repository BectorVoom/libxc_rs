//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1268/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1268(t30110: f64, t531: f64, t1913: f64, t7956: f64, t30197: f64, t571: f64, t2045: f64, t6936: f64, t1921: f64, t7939: f64, t2037: f64, t6951: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t109173 = t531 * t30110;
    let t109339 = t1913 * t7956;
    let t109345 = t571 * t30197;
    let t109348 = t6936 * t2045;
    let t109349 = t7939 * t1921;
    let t109351 = t2037 * t6951;
    (t109173, t109339, t109345, t109348, t109349, t109351)
}
