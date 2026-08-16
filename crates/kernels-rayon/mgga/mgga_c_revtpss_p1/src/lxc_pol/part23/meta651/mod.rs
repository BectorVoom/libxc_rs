//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta651 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2378;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2379;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta651(t2737: f64, t40609: f64, t2694: f64, t9789: f64, t853: f64, t9794: f64, t10292: f64, t66: f64, t240: f64, t10688: f64, t243: f64, t268: f64, t9784: f64, t16: f64, t2236: f64, t236: f64, t281: f64, t39644: f64, t10871: f64, t775: f64, t10696: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40611, t40625, t40627, t40633, t40634, t40638) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2378(t2737, t40609, t2694, t9789, t853, t9794, t10292, t66, t240, t10688, t243, t268);
        let (t40639, t40649, t40650, t40654, t40664, t40672) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2379(t2694, t9784, t16, t2236, t240, t236, t243, t281, t39644, t10871, t775, t10696, t72);
    (t40611, t40625, t40627, t40633, t40634, t40638, t40639, t40649, t40650, t40654, t40664, t40672)
}
