//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta313 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1100;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta313(t22259: f64, t4018: f64, t14045: f64, t6869: f64, t3992: f64, t2661: f64, t221: f64, t4019: f64, t6874: f64, t6864: f64, t9918: f64, t3930: f64, t6876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t22260, t22263, t22264, t22267, t22268, t22285, t22292) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1100(t22259, t4018, t14045, t6869, t3992, t2661, t221, t4019, t6874, t6864, t9918, t3930, t6876);
    (t22260, t22263, t22264, t22267, t22268, t22285, t22292)
}
