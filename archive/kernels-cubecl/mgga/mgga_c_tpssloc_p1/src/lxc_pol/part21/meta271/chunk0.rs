//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1535/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1535<F: Float>(t761: F, t9722: F, t2517: F, t718: F, t2475: F, t723: F, t159: F, t2461: F, t730: F, t167: F, t2478: F, t164: F) -> (F, F, F, F, F, F, F, F) {
    let t9724 = F::cast_from(0.10389515463408878255e3_f64) * t761 * t9722;
    let t9726 = t718 * t2517;
    let t9729 = F::cast_from(1.0_f64) / t2475 / t723;
    let t9730 = t159 * t9729;
    let t9731 = t2461 * t730;
    let t9733 = F::cast_from(1.0_f64) / t2478 / t167;
    let t9734 = t9731 * t9733;
    let t9738 = F::cast_from(1.0_f64) / t2475 / t164;
    (t9724, t9726, t9729, t9730, t9731, t9733, t9734, t9738)
}
