//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1411/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1411<F: Float>(t12168: F, t17751: F, t12366: F, t17748: F, t5186: F, t5190: F, t8582: F, t1459: F, t17423: F, t2993: F, t3018: F, t53039: F) -> (F, F, F, F, F) {
    let t59191 = F::new(24.0) * t12168 * t17751;
    let t59193 = F::cast_from(0.19298189186581325787e3_f64) * t12366 * t17748;
    let t59196 = F::cast_from(0.57894567559743977359e3_f64) * t8582 * t5190 * t5186;
    let t59199 = F::new(8.0) * t2993 * t17423 * t1459;
    let t59202 = F::cast_from(0.64327297288604419288e2_f64) * t3018 * t53039 * t1459;
    (t59191, t59193, t59196, t59199, t59202)
}
