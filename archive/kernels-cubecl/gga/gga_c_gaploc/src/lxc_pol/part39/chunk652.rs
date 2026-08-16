//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 652/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk652<F: Float>(t2541: F, t8682: F, t2508: F, t3437: F, t731: F, t10631: F, t10634: F, t10638: F, t10642: F, t9618: F, t9620: F, t9622: F, t9627: F, t9629: F, t9632: F, t9635: F, t9651: F) -> (F, F, F) {
    let t10643 = t2541 * t8682;
    let t10645 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t10643;
    let t10646 = t731 * t3437;
    let t10647 = F::cast_from(0.42725145723012357132e-3_f64) * t10646;
    let t10648 = t9618 - t9620 - t9622 - t9627 + t9629 + t9632 - t10631 + t10634 - t10638 - t10642 - t10645 - t10647 - t9635 - t9651;
    (t10645, t10647, t10648)
}
