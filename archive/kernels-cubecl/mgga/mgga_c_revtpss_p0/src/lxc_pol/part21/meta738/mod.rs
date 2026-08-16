//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta738 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2591;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta738<F: Float>(t2435: F, t9635: F, t9590: F, t9593: F, t10179: F, t1450: F, t4146: F, t1455: F, t5808: F, t46279: F, t46281: F, t46286: F) -> (F, F, F, F, F, F, F, F) {
        let (t47620, t47638, t47651, t47672, t47730, t47753, t47754, t47758) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2591::<F>(t2435, t9635, t9590, t9593, t10179, t1450, t4146, t1455, t5808, t46279, t46281, t46286);
    (t47620, t47638, t47651, t47672, t47730, t47753, t47754, t47758)
}
