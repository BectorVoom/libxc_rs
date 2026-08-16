//! MGGA_C_TPSSLOC kxc pol — kxc_pol part 4 (v3rho3_2) CSE chunk 1180/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_kxc_pol_part4_v3rho3_2_chunk1180<F: Float>(t5: F, t19448: F, t112: F, t111: F, t5449: F, t1441: F, t671: F, t5456: F, t649: F, t5465: F, t626: F, t5464: F, t9365: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t19449 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t19448);
    let t19450 = t19449 * t112;
    let t19451 = t5449 * t111;
    let t19456 = t1441 * t671;
    let t19461 = t649 * t5456;
    let t19471 = t626 * t5465;
    let t19473 = t9365 * t5464;
    (t19450, t19451, t19456, t19461, t19471, t19473)
}
