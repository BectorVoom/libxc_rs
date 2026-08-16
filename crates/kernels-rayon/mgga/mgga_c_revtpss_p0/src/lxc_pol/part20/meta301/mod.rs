//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1185;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1186;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta301(t12334: f64, t12356: f64, t1150: f64, t1131: f64, t1126: f64, t3383: f64, t3386: f64, t12228: f64, t3433: f64, t12295: f64, t12292: f64, t12297: f64, t12299: f64, t12301: f64, t12303: f64, t12307: f64, t12310: f64, t12314: f64, t12317: f64, t12320: f64, t448: f64, t300: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12357, t12358, t12360, t12361, t12363, t12364, t12366, t12378) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1185(t12334, t12356, t1150, t1131, t1126, t3383, t3386, t12228, t3433, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
        let (t12379, t12381, t12393) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1186(t12378, t448, t300, t12295, t12292, t12297, t12299, t12301, t12303, t12307, t12310, t12314, t12317, t12320);
    (t12357, t12358, t12360, t12361, t12363, t12364, t12366, t12378, t12379, t12381, t12393)
}
