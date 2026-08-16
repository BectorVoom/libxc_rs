//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3071/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3071<F: Float>(t15068: F, t51120: F, t11185: F, t18677: F, t1098: F, t18245: F, t1119: F, t18686: F, t3308: F, t3312: F, t5983: F, t3316: F) -> (F, F, F, F, F) {
    let t63745 = F::cast_from(0.1034520258385468006e4_f64) * t51120 * t15068;
    let t63747 = F::cast_from(12.0_f64) * t11185 * t18677;
    let t63750 = t18245 * t1098;
    let t63752 = F::cast_from(2.0_f64) * t63750 * t1119;
    let t63754 = F::cast_from(1.0_f64) * t18686 * t3308;
    let t63755 = t5983 * t3312;
    let t63757 = F::cast_from(0.16081979498692535067e2_f64) * t63755 * t3316;
    (t63745, t63747, t63752, t63754, t63757)
}
