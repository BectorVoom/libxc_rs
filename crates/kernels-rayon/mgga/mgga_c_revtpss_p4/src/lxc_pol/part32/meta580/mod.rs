//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta580(t26292: f64, t27899: f64, t102295: f64, t7289: f64, t1426: f64, t786: f64, t8086: f64, t3917: f64, t14090: f64, t26265: f64, t14104: f64, t96515: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t102409, t102411, t102420, t102422, t102434, t102439) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1908(t26292, t27899, t102295, t7289, t1426, t786, t8086, t3917, t14090, t26265, t14104, t96515);
    (t102409, t102411, t102420, t102422, t102434, t102439)
}
