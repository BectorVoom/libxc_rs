//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1473/1484 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1473<F: Float>(t122595: F, t122597: F, t122598: F, t122599: F, t122600: F, t122602: F, t122603: F, t122604: F, t122605: F, t122606: F, t122608: F, t26906: F, t5361: F, t7040: F, t8103: F, t8690: F, t8840: F) -> F {
    let t124994 = F::cast_from(3.0_f64) * t26906 * t8690 + t5361 * t8840 - t7040 * t8103 + t122595 - F::cast_from(2.0_f64) * t122597 - F::cast_from(2.0_f64) * t122598 - F::cast_from(2.0_f64) * t122599 - F::cast_from(2.0_f64) * t122600 - F::cast_from(2.0_f64) * t122602 - F::cast_from(2.0_f64) * t122603 - F::cast_from(2.0_f64) * t122604 - F::cast_from(2.0_f64) * t122605 - F::cast_from(2.0_f64) * t122606 - F::cast_from(2.0_f64) * t122608;
    t124994
}
