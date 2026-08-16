//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta776 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2766;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2767;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta776(t4302: f64, t9586: f64, t13312: f64, t189: f64, t4401: f64, t606: f64, t14389: f64, t2258: f64, t10612: f64, t4311: f64, t14330: f64, t14369: f64, t2251: f64, t14325: f64, t14622: f64, t40156: f64, t14440: f64, t2398: f64, t40172: f64, t40178: f64, t14370: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50857, t50861, t50864, t50866, t50868) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2766(t4302, t9586, t13312, t189, t4401, t606, t14389, t2258, t10612, t4311, t14330, t14369, t2251);
        let (t50869, t50871, t50872, t50874, t50875, t50876, t50879, t50880) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2767(t50868, t14325, t14622, t40156, t14440, t2398, t40172, t40178, t14369, t2258, t4401, t14370);
    (t50857, t50861, t50864, t50866, t50869, t50871, t50872, t50874, t50875, t50876, t50879, t50880)
}
