//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1406/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1406<F: Float>(t43503: F, t43508: F, t44329: F, t52687: F, t52689: F, t58758: F, t58761: F, t58763: F, t58765: F, t58770: F, t58774: F, t58776: F, t58778: F, t58780: F) -> F {
    let t59147 = F::new(0.46074375e0) * t58758 - F::cast_from(0.3560484375e1_f64) * t58761 - F::new(0.28483875e1) * t58763 + F::cast_from(0.1151859375e0_f64) * t58765 - F::cast_from(0.79724444444444444446e0_f64) * t43503 + F::cast_from(0.15944888888888888889e1_f64) * t43508 - F::cast_from(0.54771111111111111111e0_f64) * t44329 + F::new(0.3071625e0) * t58770 + F::cast_from(0.21908444444444444444e0_f64) * t52687 - F::cast_from(0.13145066666666666666e1_f64) * t52689 - F::new(0.379785e1) * t58774 + F::new(0.614325e0) * t58776 + F::new(0.85451625e1) * t58778 - F::new(0.46074375e0) * t58780;
    t59147
}
