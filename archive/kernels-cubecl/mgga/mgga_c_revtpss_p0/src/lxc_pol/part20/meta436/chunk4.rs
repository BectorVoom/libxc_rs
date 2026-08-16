//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1647/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1647<F: Float>(t1149: F, t12357: F, t3433: F, t3435: F, t12227: F, t12230: F, t3385: F, t3427: F, t3386: F, t1130: F, t12393: F, t1151: F) -> (F, F, F, F) {
    let t45033 = F::cast_from(0.64327917994770140268e2_f64) * t3433 * t12357 * t3435 * t1149;
    let t45037 = F::cast_from(0.3103560775156404018e4_f64) * t12227 * t3385 * t12230 * t3427;
    let t45040 = F::cast_from(36.0_f64) * t3433 * t3386 * t3427;
    let t45041 = t12393 * t1130;
    let t45043 = F::cast_from(4.0_f64) * t45041 * t1151;
    (t45033, t45037, t45040, t45043)
}
