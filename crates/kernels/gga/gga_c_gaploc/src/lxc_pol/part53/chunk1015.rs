//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1015/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1015<F: Float>(t1377: F, t14479: F, t47071: F, t41595: F, t41600: F, t41604: F, t41607: F, t41609: F, t41612: F, t41616: F, t41619: F, t41624: F, t41627: F, t41630: F, t41646: F, t47794: F, t47800: F, t47805: F, t47808: F, t47812: F, t47823: F, t47827: F) -> (F, F, F) {
    let t50809 = t1377 * t14479;
    let t50811 = F::new(2.0) * t47071;
    let t50820 = -F::new(0.51123901271894332902e1) * t47794 - t41595 + t41600 - t41604 + F::new(0.20449560508757733161e1) * t47800 - t41607 - F::new(0.1533717038156829987e1) * t41609 + F::new(0.72851559312449424384e1) * t41612 + t41616 - t41619 - F::new(0.18404604457881959845e2) * t47805 + F::new(0.87421871174939309263e2) * t47808 + F::new(0.30674340763136599742e1) * t47812 + t41624 + t41627 + t41630 + t41646 + t47823 - t47827;
    (t50809, t50811, t50820)
}
