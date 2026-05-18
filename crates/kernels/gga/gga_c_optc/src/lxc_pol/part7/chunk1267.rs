//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1267/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1267<F: Float>(t2994: F, t1056: F, t8582: F, t3012: F, t2993: F, t3018: F, t3020: F, t2917: F) -> (F, F, F, F, F) {
    let t26153 = t2994 * t2994;
    let t26156 = F::new(24.0) * t8582 * t26153 * t1056;
    let t26157 = t3012 * t3012;
    let t26160 = F::new(6.0) * t2993 * t26157 * t1056;
    let t26163 = F::new(0.48245472966453314466e2) * t3018 * t26157 * t3020;
    let t26164 = t2917 * t2917;
    (t26153, t26156, t26160, t26163, t26164)
}
