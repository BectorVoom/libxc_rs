//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2990/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2990<F: Float>(t17624: F, t2960: F, t5884: F, t698: F, t973: F, t5889: F, t10876: F, t10937: F, t10949: F, t13980: F, t13985: F, t13995: F, t14069: F, t17637: F, t17670: F, t17681: F, t17714: F, t3117: F, t43385: F, t4582: F, t50084: F, t50094: F, t50098: F, t50100: F, t50110: F, t50113: F, t50116: F) -> F {
    let t62556 = t2960 * t17624;
    let t62559 = t973 * t698 * t5884;
    let t62565 = t973 * t698 * t5889;
    let t62576 = -t50084 / F::cast_from(1728.0_f64) - t10876 * t4582 * t17670 * t13980 / F::cast_from(512.0_f64) - F::cast_from(3.0_f64) / F::cast_from(256.0_f64) * t43385 * t4582 * t17670 * t13985 - t3117 * t17637 / F::cast_from(1152.0_f64) + t62556 / F::cast_from(81.0_f64) + t62559 / F::cast_from(648.0_f64) + t10949 * t17714 / F::cast_from(768.0_f64) + t50094 / F::cast_from(1728.0_f64) - t62565 / F::cast_from(1296.0_f64) + F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t50098 + t13995 * t14069 / F::cast_from(1152.0_f64) - t10937 * t17681 / F::cast_from(432.0_f64) + t50100 / F::cast_from(216.0_f64) + t50110 / F::cast_from(162.0_f64) + t50113 / F::cast_from(324.0_f64) + F::cast_from(7.0_f64) / F::cast_from(972.0_f64) * t50116;
    t62576
}
