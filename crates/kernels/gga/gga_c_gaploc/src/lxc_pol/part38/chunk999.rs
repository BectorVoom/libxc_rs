//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 999/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk999<F: Float>(t13332: F, t501: F, t605: F, t11718: F, t7324: F, t13343: F, t17293: F, t13581: F, t13718: F, t1955: F, t45997: F, t46000: F, t46001: F, t46004: F, t46006: F, t46008: F, t46011: F, t46013: F, t46016: F, t46019: F, t46023: F, t46025: F, t46828: F, t5552: F, t841: F) -> (F, F, F) {
    let t46829 = t13332 * t501;
    let t46830 = t46829 * t605;
    let t46832 = F::new(2.0) * t7324 * t11718;
    let t46835 = F::new(24.0) * t17293 * t13343 * t605;
    let t46836 = F::new(4.0) * t13581 * t5552 - t13718 * t1955 - t46001 * t841 - t45997 - t46000 + t46004 - t46006 + t46008 + t46011 + t46013 - t46016 + t46019 - t46023 - t46025 + t46828 + t46830 + t46832 - t46835;
    (t46830, t46835, t46836)
}
