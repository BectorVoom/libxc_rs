//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 839/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk839<F: Float>(t13516: F, t64: F, t40612: F, t40614: F, t40620: F, t40630: F, t40632: F, t40634: F, t43027: F, t13624: F, t1841: F, t2536: F, t734: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44857 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13516 * t64;
    let t44860 = F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t40612;
    let t44861 = F::cast_from(63.0_f64) / F::cast_from(8192.0_f64) * t40614;
    let t44862 = F::cast_from(63.0_f64) / F::cast_from(524288.0_f64) * t40620;
    let t44863 = F::cast_from(21.0_f64) / F::cast_from(524288.0_f64) * t40630;
    let t44864 = F::cast_from(21.0_f64) / F::cast_from(8192.0_f64) * t40632;
    let t44865 = F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t40634;
    let t44883 = F::cast_from(0.1281754371690370714e-2_f64) * t43027;
    let t44887 = F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t2536 * t13624 * t734;
    (t44857, t44860, t44861, t44862, t44863, t44864, t44865, t44883, t44887)
}
