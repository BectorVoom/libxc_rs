//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta637 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2410;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2411;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta637(t11010: f64, t689: f64, t779: f64, t2769: f64, t786: f64, t861: f64, t10997: f64, t11007: f64, t252: f64, t11009: f64, t123: f64, t676: f64, t11006: f64, t256: f64, t225: f64, t2782: f64, t2828: f64, t886: f64, t2441: f64, t39515: f64, t10504: f64, t138: f64, t9302: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41063, t41066, t41067, t41070, t41073) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2410(t11010, t689, t779, t2769, t786, t861, t10997, t11007, t252, t11009, t123, t676);
        let (t41078, t41092, t41095, t41098) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2411(t11006, t256, t225, t252, t2769, t2782, t2828, t886, t2441, t39515, t10504, t138, t9302);
    (t41063, t41066, t41067, t41070, t41073, t41078, t41092, t41095, t41098)
}
