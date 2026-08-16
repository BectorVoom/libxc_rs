//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta157 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk790;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk791;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk792;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta157(t373: f64, t6305: f64, t3155: f64, t1042: f64, t3162: f64, t225: f64, t6235: f64, t366: f64, t1066: f64, t6100: f64, t247: f64, t3182: f64, t6092: f64, t6096: f64, t6244: f64, t371: f64, t372: f64, t1041: f64, t1063: f64, t1671: f64, t1675: f64, t3150: f64, t3161: f64, t3203: f64, t3205: f64, t375: f64, t4834: f64, t4846: f64, t4879: f64, t4925: f64, t6302: f64, t6298: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6326) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk790(t373, t6305, t3155, t1042, t3162, t225, t6235, t366, t1066, t6100, t247, t3182, t6092);
        let (t6327, t6331, t6337, t6339, t6342) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk791(t247, t6326, t1066, t6096, t373, t6244, t371, t372, t1041, t1063, t1671, t1675, t3150, t3161, t3203, t3205, t375, t4834, t4846, t4879, t4925, t6302, t6308, t6312, t6318, t6323);
        let t6343 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk792(t6298, t6342);
    (t6307, t6308, t6311, t6312, t6317, t6318, t6323, t6327, t6331, t6337, t6339, t6343)
}
