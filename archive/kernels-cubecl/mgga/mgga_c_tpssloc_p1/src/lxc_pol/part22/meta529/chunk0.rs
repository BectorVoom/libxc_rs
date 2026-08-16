//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2000/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2000<F: Float>(t21: F, t9: F, t587: F, t598: F, t14: F, t2230: F, t594: F, t9223: F, t22811: F, t19: F, t601: F, t9238: F) -> (F, F, F, F, F, F) {
    let t39033 = t9 * t21;
    let t39035 = t587 * t598;
    let t39037 = t14 * t2230;
    let t39039 = t594 * t9223;
    let t39041 = F::cast_from(1.0_f64) / t22811;
    let t39043 = F::cast_from(0.683424e4_f64) * t19 * t39041;
    let t39054 = t601 * t9238;
    (t39033, t39035, t39037, t39039, t39043, t39054)
}
