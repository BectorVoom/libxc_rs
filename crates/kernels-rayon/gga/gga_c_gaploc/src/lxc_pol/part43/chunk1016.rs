//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 1016/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk1016(t41595: f64, t41600: f64, t41604: f64, t41607: f64, t41609: f64, t41612: f64, t41616: f64, t41619: f64, t41624: f64, t41627: f64, t41630: f64, t41646: f64, t47794: f64, t47800: f64, t47805: f64, t47808: f64, t47812: f64, t47823: f64, t47827: f64) -> f64 {
    let t50820 = -0.51123901271894332902e1_f64 * t47794 - t41595 + t41600 - t41604 + 0.20449560508757733161e1_f64 * t47800 - t41607 - 0.1533717038156829987e1_f64 * t41609 + 0.72851559312449424384e1_f64 * t41612 + t41616 - t41619 - 0.18404604457881959845e2_f64 * t47805 + 0.87421871174939309263e2_f64 * t47808 + 0.30674340763136599742e1_f64 * t47812 + t41624 + t41627 + t41630 + t41646 + t47823 - t47827;
    t50820
}
