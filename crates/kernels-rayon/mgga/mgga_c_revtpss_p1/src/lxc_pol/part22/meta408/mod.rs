//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta408 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2005;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2006;
use chunk2::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2007;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta408(t5710: f64, t72: f64, t1432: f64, t686: f64, t136: f64, t1892: f64, t2457: f64, t3964: f64, t2435: f64, t5760: f64, t3999: f64, t545: f64, t869: f64, t689: f64, t225: f64, t9990: f64, t213: f64, t10062: f64, t10130: f64, t13805: f64, t1399: f64, t14122: f64, t14127: f64, t1883: f64, t3924: f64, t4004: f64, t4057: f64, t5675: f64, t5735: f64, t5745: f64, t5755: f64, t5767: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14155, t14158, t14159, t14161, t14166, t14171, t14188) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2005(t5710, t72, t1432, t686, t136, t1892, t2457, t3964, t2435, t5760, t3999, t545);
        let (t14189, t14191, t14192, t14193) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2006(t14188, t869, t689, t225, t9990, t213);
        let t14200 = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2007(t10062, t10130, t13805, t1399, t14122, t14127, t14158, t14161, t14166, t14171, t14191, t14193, t1883, t3924, t4004, t4057, t5675, t5735, t5745, t5755, t5767, t820);
    (t14155, t14158, t14159, t14161, t14166, t14171, t14188, t14189, t14191, t14192, t14193, t14200)
}
