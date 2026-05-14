//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 224/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk224<F: Float>(t829: F, t831: F, t766: F, t798: F, t297: F, t332: F, t268: F, t9: F, t22: F, t760: F, t768: F, t786: F, t159: F, t751: F, t104: F, t260: F) -> (F, F, F, F, F, F, F, F) {
    let t832 = t829 * t831;
    let t835 = t766 * t798;
    let t836 = t297 * t332;
    let t837 = t9 * t268;
    let t841 = t22 * t760;
    let t845 = t768 * t786;
    let t849 = t751 * t159;
    let t852 = t260 * t104;
    (t832, t835, t836, t837, t841, t845, t849, t852)
}
