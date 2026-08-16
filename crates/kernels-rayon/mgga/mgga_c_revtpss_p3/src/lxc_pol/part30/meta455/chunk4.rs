//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1736/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1736(t17186: f64, t17859: f64, t17912: f64, t17961: f64, t1277: f64, t1828: f64, t3738: f64, t13182: f64, t3566: f64, t488: f64, t1276: f64, t1774: f64) -> (f64, f64, f64, f64, f64) {
    let t17963 = t17186 + t17859 + t17912 + t17961;
    let t17964 = t1277 * t17963;
    let t17967 = t1828 * t3738;
    let t17968 = t13182 * t17967;
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    (t17963, t17964, t17968, t17973, t17974)
}
