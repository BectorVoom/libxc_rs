//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1348/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1348<F: Float>(t761: F, t9722: F, t9450: F, t9457: F, t9463: F, t9469: F, t9476: F, t9484: F, t9496: F, t9684: F, t9715: F, t9718: F) -> (F, F) {
    let t9724 = F::cast_from(0.10389515463408878255e3_f64) * t761 * t9722;
    let t9725 = t9450 - t9457 + t9463 - t9469 + t9476 + t9484 - t9496 + t9684 - t9715 - t9718 + t9724;
    (t9724, t9725)
}
