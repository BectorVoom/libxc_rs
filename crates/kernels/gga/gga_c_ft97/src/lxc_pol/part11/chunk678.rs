//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 678/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk678<F: Float>(t9768: F, t9765: F, t2475: F, t747: F, t2514: F, t91: F, t251: F, t631: F, t675: F, t7242: F, t898: F, t2476: F, t2371: F, t665: F) -> (F, F, F, F, F, F, F) {
    let t9872 = 2.0 / 9.0 * t9768;
    let t9876 = 2.0 / 9.0 * t9765;
    let t9881 = t2475 * t747;
    let t9883 = t91 * t9881 * t2514;
    let t9890 = 1.0 / t251 / t631 / t898 / t675 / t7242 / 4.0;
    let t9891 = t2476 * t747;
    let t9893 = t91 * t9890 * t9891;
    let t9895 = t665 * t2371;
    (t9872, t9876, t9881, t9883, t9890, t9893, t9895)
}
