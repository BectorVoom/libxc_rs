//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 887/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk887<F: Float>(t1960: F, t2728: F, t3684: F, t11711: F, t23555: F, t10298: F, t8045: F, t2902: F, t3366: F, t4349: F, t1052: F, t11125: F, t13581: F, t13718: F, t2972: F, t3073: F, t331: F, t33992: F, t34013: F, t3511: F, t44749: F, t44794: F, t44845: F, t44917: F, t44964: F, t45016: F, t45070: F, t45116: F, t45123: F, t45124: F, t45126: F, t45130: F, t45132: F, t45134: F, t45141: F, t5559: F, t841: F) -> (F, F, F) {
    let t45144 = F::cast_from(2.0_f64) * t1960 * t3684 * t2728;
    let t45146 = F::cast_from(6.0_f64) * t23555 * t11711;
    let t45148 = F::cast_from(4.0_f64) * t8045 * t10298;
    let t45151 = F::cast_from(12.0_f64) * t4349 * t3366 * t2902;
    let t45161 = (t44749 + t44794 + t44845 + t44917 + t44964 + t45016 + t45070 + t45116) * t331 - t45123 - t45124 + t45126 - F::cast_from(2.0_f64) * t34013 * t1052 - t45130 + t45132 - t45134 + F::cast_from(4.0_f64) * t33992 * t2972 + F::cast_from(4.0_f64) * t1960 * t3073 * t3511 + t45141 + t45144 - t45146 - t45148 + t45151 - F::cast_from(12.0_f64) * t5559 * t13581 * t841 + F::cast_from(2.0_f64) * t1960 * t13718 * t841 + F::cast_from(4.0_f64) * t1960 * t1052 * t11125;
    (t45148, t45151, t45161)
}
