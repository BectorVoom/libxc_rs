//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta529 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1564;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1565;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta529(t17605: f64, t21090: f64, t127: f64, t12988: f64, t24617: f64, t371: f64, t20842: f64, t5323: f64, t1010: f64, t22700: f64, t21169: f64, t5373: f64, t21251: f64, t1219: f64, t24551: f64, t21254: f64, t12772: f64, t24797: f64, t3625: f64, t1256: f64, t24684: f64, t24700: f64, t1803: f64, t20850: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t83916, t83920, t83922, t83962, t83992) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1564(t17605, t21090, t127, t12988, t24617, t371, t20842, t5323, t1010, t22700, t21169, t5373);
        let (t83994, t84029, t84032, t84061, t84082, t84084, t84098) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1565(t21251, t5373, t1219, t24551, t21254, t12772, t24797, t3625, t1256, t24684, t24700, t1803, t20850);
    (t83916, t83920, t83922, t83962, t83992, t83994, t84029, t84032, t84061, t84082, t84084, t84098)
}
