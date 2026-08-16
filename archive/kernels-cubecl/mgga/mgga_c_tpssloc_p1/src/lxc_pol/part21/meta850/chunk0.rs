//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3078/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3078<F: Float>(t18746: F, t3279: F, t14758: F, t4764: F, t1102: F, t18730: F, t3287: F, t18751: F, t18754: F, t4748: F, t3270: F, t18761: F) -> (F, F, F, F, F, F, F, F) {
    let t63848 = t18746 * t3279;
    let t63853 = t4764 * t14758;
    let t63856 = t3287 * t18730 * t1102;
    let t63858 = t18751 * t3279;
    let t63860 = t18754 * t3279;
    let t63862 = t4748 * t14758;
    let t63865 = t3270 * t18730 * t1102;
    let t63867 = t18761 * t3279;
    (t63848, t63853, t63856, t63858, t63860, t63862, t63865, t63867)
}
