//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1180/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1180<F: Float>(t763: F, t9716: F, t177: F, t2508: F, t2512: F, t9490: F, t761: F, t2517: F, t718: F, t2475: F, t723: F, t159: F) -> (F, F, F, F, F, F) {
    let t9717 = t9716 * t763;
    let t9720 = F::cast_from(1.0_f64) / t2508 / t177;
    let t9722 = t9720 * t9490 * t2512;
    let t9724 = F::cast_from(0.10389515463408878255e3_f64) * t761 * t9722;
    let t9726 = t718 * t2517;
    let t9729 = F::cast_from(1.0_f64) / t2475 / t723;
    let t9730 = t159 * t9729;
    (t9717, t9720, t9722, t9724, t9726, t9730)
}
