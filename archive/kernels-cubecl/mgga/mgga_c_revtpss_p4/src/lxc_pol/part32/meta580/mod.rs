//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta580 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1908;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta580<F: Float>(t26292: F, t27899: F, t102295: F, t7289: F, t1426: F, t786: F, t8086: F, t3917: F, t14090: F, t26265: F, t14104: F, t96515: F) -> (F, F, F, F, F, F) {
        let (t102409, t102411, t102420, t102422, t102434, t102439) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1908::<F>(t26292, t27899, t102295, t7289, t1426, t786, t8086, t3917, t14090, t26265, t14104, t96515);
    (t102409, t102411, t102420, t102422, t102434, t102439)
}
