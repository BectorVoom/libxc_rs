//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1365;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1366;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta418(t342: f64, t43471: f64, t3154: f64, t43351: f64, t16551: f64, t994: f64, t16558: f64, t11627: f64, t42859: f64, t11631: f64, t3494: f64, t3519: f64, t13026: f64, t240: f64, t3361: f64, t2304: f64, t25273: f64, t268: f64, t404: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t43472, t43473, t43520, t43524, t43537, t43538, t43752) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1365(t342, t43471, t3154, t43351, t16551, t994, t16558, t11627, t42859, t11631, t3494, t3519);
        let (t43764, t43766, t43776, t43813) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1366(t13026, t240, t3361, t2304, t25273, t268, t404);
    (t43472, t43473, t43520, t43524, t43537, t43538, t43752, t43764, t43766, t43776, t43813)
}
