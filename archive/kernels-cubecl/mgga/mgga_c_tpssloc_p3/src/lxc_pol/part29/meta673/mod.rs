//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2260;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta673<F: Float>(t22574: F, t56120: F, t8643: F, t1845: F, t3719: F, t1874: F, t55962: F, t19456: F, t6525: F, t22480: F, t4028: F, t26502: F, t532: F, t1983: F, t6879: F, t2314: F, t26142: F, t4034: F, t1266: F, t26135: F, t652: F, t24987: F, t6997: F, t22591: F, t24990: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t91602, t91606, t91608, t91610, t91612, t91620) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2260::<F>(t22574, t56120, t8643, t1845, t3719, t1874, t55962, t19456, t6525, t22480, t4028, t26502, t532);
        let (t91623, t91625, t91627, t91630, t91637, t91640) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2261::<F>(t1983, t6879, t91620, t2314, t26142, t4034, t1266, t26135, t652, t24987, t6997, t22591, t24990);
    (t91602, t91606, t91608, t91610, t91612, t91623, t91625, t91627, t91630, t91637, t91640)
}
