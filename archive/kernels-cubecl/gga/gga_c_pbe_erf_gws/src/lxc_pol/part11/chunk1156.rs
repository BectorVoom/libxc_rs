//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1156/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1156<F: Float>(t12723: F, t7130: F, t41133: F, t5211: F, t7491: F, t954: F, t12782: F, t7115: F, t7117: F, t42142: F, t33281: F, t184: F, t221: F, t3477: F, t3491: F) -> (F, F, F, F, F, F) {
    let t48373 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t7130 * t12723;
    let t48377 = F::cast_from(32.0_f64) / F::cast_from(9.0_f64) * t5211 * t7491 * t41133 * t954;
    let t48380 = F::cast_from(32.0_f64) / F::cast_from(15.0_f64) * t7115 * t7117 * t12782;
    let t48381 = F::cast_from(16.0_f64) / F::cast_from(15.0_f64) * t42142;
    let t48382 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t33281;
    let t48387 = F::cast_from(8.0_f64) / F::cast_from(5.0_f64) * t3491 * t3477 * t184 * t221;
    (t48373, t48377, t48380, t48381, t48382, t48387)
}
