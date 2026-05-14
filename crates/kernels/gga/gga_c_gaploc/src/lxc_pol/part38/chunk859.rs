//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 859/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk859<F: Float>(t13722: F, t45134: F, t45148: F, t45151: F, t45164: F, t45971: F, t45973: F, t45974: F, t45978: F, t45986: F, t45992: F, t46000: F, t46006: F, t46011: F, t46023: F, t46025: F, t46828: F, t46830: F, t46835: F, t617: F) -> (F,) {
    let t46842 = t13722 * t617 + t45134 + t45148 - t45151 - t45164 + t45971 + t45973 - t45974 - t45978 + t45986 + t45992 + t46000 + t46006 - t46011 + t46023 + t46025 - t46828 - t46830 + t46835;
    (t46842,)
}
