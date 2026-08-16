//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2766;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta776<F: Float>(t4302: F, t9586: F, t13312: F, t189: F, t4401: F, t606: F, t14389: F, t2258: F, t10612: F, t4311: F, t14330: F, t14369: F, t2251: F, t14325: F, t14622: F, t40156: F, t14440: F, t2398: F, t40172: F, t40178: F, t14370: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t50857, t50861, t50864, t50866, t50868) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2766::<F>(t4302, t9586, t13312, t189, t4401, t606, t14389, t2258, t10612, t4311, t14330, t14369, t2251);
        let (t50869, t50871, t50872, t50874, t50875, t50876, t50879, t50880) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2767::<F>(t50868, t14325, t14622, t40156, t14440, t2398, t40172, t40178, t14369, t2258, t4401, t14370);
    (t50857, t50861, t50864, t50866, t50869, t50871, t50872, t50874, t50875, t50876, t50879, t50880)
}
