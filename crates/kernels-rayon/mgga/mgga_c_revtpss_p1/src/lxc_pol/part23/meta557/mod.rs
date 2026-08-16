//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta557 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2116;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2117;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta557(t1444: f64, t6895: f64, t9657: f64, t22307: f64, t225: f64, t212: f64, t6888: f64, t1358: f64, t689: f64, t1357: f64, t6896: f64, t72: f64, t686: f64, t9680: f64, t10160: f64, t10163: f64, t10166: f64, t1424: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t213: f64, t4071: f64, t561: f64, t6919: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22432, t22433, t22441, t22445, t22446, t22447, t22449, t22450, t22452) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2116(t1444, t6895, t9657, t22307, t225, t212, t6888, t1358, t689, t1357, t6896, t72);
        let (t22453, t22454, t22459) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2117(t22452, t686, t9680, t10160, t10163, t10166, t1424, t14280, t14290, t14294, t14297, t213, t22433, t22441, t22447, t22450, t4071, t561, t6919);
    (t22432, t22433, t22441, t22445, t22446, t22447, t22449, t22450, t22452, t22453, t22454, t22459)
}
