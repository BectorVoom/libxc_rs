//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta625 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2386;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2387;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta625(t10292: f64, t65: f64, t235: f64, t2710: f64, t826: f64, t225: f64, t785: f64, t2737: f64, t2694: f64, t9789: f64, t853: f64, t9794: f64, t775: f64, t837: f64, t10760: f64, t66: f64, t240: f64, t10688: f64, t243: f64, t268: f64, t9784: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40604, t40607, t40609, t40611, t40625, t40627) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2386(t10292, t65, t235, t2710, t826, t225, t785, t2737, t2694, t9789, t853, t9794);
        let (t40628, t40630, t40633, t40634, t40638, t40639) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2387(t775, t837, t10760, t40627, t10292, t66, t240, t10688, t243, t268, t2694, t9784);
    (t40604, t40607, t40609, t40611, t40625, t40627, t40628, t40630, t40633, t40634, t40638, t40639)
}
