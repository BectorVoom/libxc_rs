//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta737 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2512;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2513;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta737(t10565: f64, t1532: f64, t4398: f64, t9419: f64, t14362: f64, t9572: f64, t37: f64, t4391: f64, t14728: f64, t9775: f64, t1549: f64, t40861: f64, t14779: f64, t40721: f64, t221: f64, t40724: f64, t14495: f64, t40834: f64, t826: f64, t241: f64, t820: f64, t849: f64, t10886: f64, t14652: f64, t808: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50892, t50893, t50901, t50903, t50939, t50941) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2512(t10565, t1532, t4398, t9419, t14362, t9572, t37, t4391, t14728, t9775, t1549, t40861);
        let (t50943, t50945, t50955, t50957, t50977) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2513(t14779, t40721, t221, t40724, t14495, t40834, t826, t241, t820, t849, t10886, t14652, t808);
    (t50892, t50893, t50901, t50903, t50939, t50941, t50943, t50945, t50955, t50957, t50977)
}
