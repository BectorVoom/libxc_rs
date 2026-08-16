//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta327 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1136;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1137;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1138;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta327(t22213: f64, t13666: f64, t13668: f64, t13670: f64, t13887: f64, t9524: f64, t9542: f64, t9588: f64, t9598: f64, t9854: f64, t9857: f64, t9865: f64, t9868: f64, t225: f64, t22917: f64, t22923: f64, t22927: f64, t22813: f64, t9880: f64, t5651: f64, t6816: f64, t1394: f64, t22809: f64, t1877: f64, t1879: f64, t539: f64, t541: f64, t5650: f64, t6832: f64, t6837: f64, t6840: f64, t543: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22928, t22929, t22930, t22931, t22932, t22933) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1136(t22213, t13666, t13668, t13670, t13887, t9524, t9542, t9588, t9598, t9854, t9857, t9865, t9868);
        let (t22936, t22944, t22947, t22950, t22953) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1137(t225, t22917, t22923, t22927, t22933, t22813, t9880, t5651, t6816, t1394, t22809, t1877, t1879, t539, t541, t5650, t6832, t6837, t6840);
        let t22954 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1138(t22953, t543);
    (t22928, t22929, t22930, t22931, t22932, t22936, t22944, t22947, t22950, t22953, t22954)
}
