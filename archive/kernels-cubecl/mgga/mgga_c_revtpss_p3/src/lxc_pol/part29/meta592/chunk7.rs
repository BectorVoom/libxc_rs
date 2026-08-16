//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1974/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1974<F: Float>(t26292: F, t27899: F, t102295: F, t7289: F, t1426: F, t786: F, t8086: F, t3917: F, t14090: F, t26265: F, t14104: F, t96515: F) -> (F, F, F, F, F) {
    let t102409 = t27899 * t26292;
    let t102411 = t7289 * t102295;
    let t102420 = t786 * t8086 * t1426;
    let t102422 = F::cast_from(0.19514881078765566038e-1_f64) * t102420 * t3917;
    let t102434 = t26265 * t14090;
    let t102439 = t96515 * t14104;
    (t102409, t102411, t102422, t102434, t102439)
}
