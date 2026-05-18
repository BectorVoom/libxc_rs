//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 1157/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk1157<F: Float>(t10530: F, t1882: F, t10522: F, t10526: F, t10700: F, t2846: F, t8232: F, t313: F, t41743: F, t89: F, t295: F, t41752: F) -> (F, F, F, F, F, F, F) {
    let t44393 = t1882 * t10530;
    let t44395 = t1882 * t10522;
    let t44397 = t1882 * t10526;
    let t44426 = t1882 * t10700;
    let t44428 = t8232 * t2846;
    let t44436 = F::new(280.0) / F::new(243.0) * t89 * t41743 * t313;
    let t44445 = t41752 * t295;
    (t44393, t44395, t44397, t44426, t44428, t44436, t44445)
}
