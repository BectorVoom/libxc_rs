//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 394/1049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk394<F: Float>(t374: F, t486: F, t677: F, t485: F, t221: F, t3426: F, t456: F, t1176: F, t3247: F, t3242: F, t3439: F, t121: F) -> (F, F, F, F, F) {
    let t3540 = t374 * t677 * t486;
    let t3542 = t485 * t3540 / F::cast_from(13824.0_f64);
    let t3545 = t221 * t3426;
    let t3547 = t456 * t3545 / F::cast_from(432.0_f64);
    let t3555 = t1176 * t3247;
    let t3560 = t3439 * t3242;
    let t3570 = t121 * t486;
    (t3542, t3547, t3555, t3560, t3570)
}
