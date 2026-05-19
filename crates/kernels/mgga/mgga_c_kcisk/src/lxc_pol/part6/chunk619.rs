//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 619/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk619<F: Float>(t1664: F, t8573: F, t1645: F, t4744: F, t8549: F, t4742: F, t4748: F, t6756: F, t8512: F, t8516: F, t8520: F, t600: F) -> (F, F, F, F, F, F) {
    let t8574 = t8573 * t1664;
    let t8576 = F::new(1.0) * t1645 * t8574;
    let t8577 = t8549 * t4744;
    let t8579 = F::cast_from(0.16081824322151104822e2_f64) * t4742 * t8577;
    let t8584 = t4748 + F::cast_from(0.61805555555555555556e-2_f64) * t6756 - F::cast_from(0.61805555555555555555e-2_f64) * t8512 + F::cast_from(0.18541666666666666667e-1_f64) * t8516 - F::cast_from(0.92708333333333333333e-2_f64) * t8520;
    let t8585 = t8584 * t600;
    (t8574, t8576, t8577, t8579, t8584, t8585)
}
