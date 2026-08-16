//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1194/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1194<F: Float>(t3375: F, t4857: F, t1157: F, t1164: F, t3400: F, t4883: F, t3411: F, t4884: F, t225: F, t4947: F, t4943: F, t1734: F, t3590: F) -> (F, F, F, F, F, F) {
    let t14960 = t3375 * t4857;
    let t14961 = t14960 * t1157;
    let t14963 = F::cast_from(0.23392894490538584828e1_f64) * t1164 * t14961;
    let t14966 = t3400 * t4857;
    let t14967 = t14966 * t4883;
    let t14969 = F::cast_from(0.34631718211362927518e2_f64) * t1164 * t14967;
    let t14971 = F::cast_from(0.34631718211362927518e2_f64) * t3411 * t4884;
    let t14972 = t4947 * t225;
    let t14980 = t4943 * t225;
    let t14985 = t3590 * t1734;
    (t14963, t14969, t14971, t14972, t14980, t14985)
}
