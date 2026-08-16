//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1305;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1306;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1307;
use chunk3::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1308;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta359(t234: f64, t251: f64, t268: f64, t39644: f64, t8779: f64, t39497: f64, t874: f64, t875: f64, t2718: f64, t2760: f64, t10530: f64, t2723: f64, t39583: f64, t10657: f64, t2646: f64, t2724: f64, t39622: f64, t39624: f64, t39629: f64, t39633: f64, t39635: f64, t39640: f64, t820: f64, t231: f64, t2798: f64, t39599: f64, t10535: f64, t281: f64, t624: f64, t836: f64, t2722: f64, t68: f64, t10529: f64, t2453: f64, t10523: f64, t10542: f64, t10960: f64, t2435: f64, t2482: f64, t39620: f64, t686: f64, t72: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t39649, t39652, t39656, t39662) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1305(t234, t251, t268, t39644, t8779, t39497, t874, t875, t2718, t2760, t10530, t2723, t39583);
        let t39664 = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1306(t10657, t2646, t2724, t39622, t39624, t39629, t39633, t39635, t39640, t39649, t39652, t39656, t39662, t820);
        let (t39668, t39673, t39675, t39678, t39680) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1307(t231, t268, t2798, t39599, t10535, t281, t624, t836, t2722, t68, t10529, t2453);
        let (t39683, t39685, t39687, t39692) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1308(t2723, t281, t39675, t39680, t10523, t10542, t10960, t2435, t2482, t39620, t686, t72, t879);
    (t39664, t39668, t39673, t39678, t39683, t39685, t39687, t39692)
}
