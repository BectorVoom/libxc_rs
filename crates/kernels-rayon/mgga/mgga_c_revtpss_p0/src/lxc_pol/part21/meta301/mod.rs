//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1551;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1552;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta301(t10605: f64, t2612: f64, t2523: f64, t2626: f64, t760: f64, t9425: f64, t2609: f64, t606: f64, t706: f64, t10592: f64, t10594: f64, t10596: f64, t10598: f64, t10602: f64, t10604: f64, t9542: f64, t10550: f64, t10571: f64, t10590: f64, t225: f64, t2475: f64, t73: f64, t2394: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10607, t10608, t10609, t10611, t10612, t10613, t10614, t10615) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1551(t10605, t2612, t2523, t2626, t760, t9425, t2609, t606, t706, t10592, t10594, t10596, t10598, t10602, t10604, t9542);
        let (t10618, t10626, t10627) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1552(t10550, t10571, t10590, t10615, t225, t2475, t73, t2394, t775);
    (t10607, t10608, t10609, t10611, t10612, t10613, t10614, t10618, t10626, t10627)
}
