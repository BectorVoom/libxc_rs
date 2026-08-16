//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1101/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1101<F: Float>(t5: F, t39221: F, t112: F, t2363: F, t111: F, t9346: F, t2405: F, t2420: F, t702: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t39222 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t39221);
    let t39223 = t39222 * t112;
    let t39231 = t2363 * t2363;
    let t39235 = t9346 * t111;
    let t39246 = t2405 * t2405;
    let t39249 = F::cast_from(6.0_f64) * t2420 * t39246 * t702;
    (t39223, t39231, t39235, t39246, t39249)
}
