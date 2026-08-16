//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 682/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk682<F: Float>(t11025: F, t7775: F, t8192: F, t7773: F, t89: F, t921: F, t3104: F, t375: F, t1636: F, t943: F, t3056: F, t77: F) -> (F, F, F, F, F, F, F, F) {
    let t11026 = F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t11025;
    let t11027 = F::cast_from(4.0_f64) / F::cast_from(81.0_f64) * t7775;
    let t11031 = F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t8192;
    let t11043 = t89 * t7773 * t921;
    let t11069 = t89 * t375 * t3104;
    let t11070 = t11069 / F::cast_from(9.0_f64);
    let t11076 = t89 * t1636 * t943;
    let t11135 = t77 * t3056;
    (t11026, t11027, t11031, t11043, t11069, t11070, t11076, t11135)
}
