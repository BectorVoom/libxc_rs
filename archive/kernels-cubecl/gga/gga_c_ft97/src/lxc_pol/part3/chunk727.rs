//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 727/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk727<F: Float>(t14635: F, t1882: F, t4041: F, t4034: F, t4053: F, t4057: F, t681: F, t89: F, t10400: F, t10279: F, t1186: F, t9733: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t14636 = t14635 / F::cast_from(27.0_f64);
    let t14637 = t1882 * t4041;
    let t14638 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t14637;
    let t14639 = t1882 * t4034;
    let t14640 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t14639;
    let t14657 = t1882 * t4053;
    let t14658 = t14657 / F::cast_from(27.0_f64);
    let t14683 = t89 * t681 * t4057;
    let t14684 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t14683;
    let t14708 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t10400;
    let t14711 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t10279;
    let t14715 = t89 * t9733 * t1186;
    (t14636, t14637, t14638, t14639, t14640, t14657, t14658, t14683, t14684, t14708, t14711, t14715)
}
