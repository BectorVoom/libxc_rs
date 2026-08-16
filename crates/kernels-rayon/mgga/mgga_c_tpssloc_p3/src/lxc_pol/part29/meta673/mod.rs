//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta673 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2260;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2261;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta673(t22574: f64, t56120: f64, t8643: f64, t1845: f64, t3719: f64, t1874: f64, t55962: f64, t19456: f64, t6525: f64, t22480: f64, t4028: f64, t26502: f64, t532: f64, t1983: f64, t6879: f64, t2314: f64, t26142: f64, t4034: f64, t1266: f64, t26135: f64, t652: f64, t24987: f64, t6997: f64, t22591: f64, t24990: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t91602, t91606, t91608, t91610, t91612, t91620) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2260(t22574, t56120, t8643, t1845, t3719, t1874, t55962, t19456, t6525, t22480, t4028, t26502, t532);
        let (t91623, t91625, t91627, t91630, t91637, t91640) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2261(t1983, t6879, t91620, t2314, t26142, t4034, t1266, t26135, t652, t24987, t6997, t22591, t24990);
    (t91602, t91606, t91608, t91610, t91612, t91623, t91625, t91627, t91630, t91637, t91640)
}
