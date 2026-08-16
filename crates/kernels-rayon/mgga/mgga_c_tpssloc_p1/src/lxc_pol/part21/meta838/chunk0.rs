//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2990/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2990(t17624: f64, t2960: f64, t5884: f64, t698: f64, t973: f64, t5889: f64, t10876: f64, t10937: f64, t10949: f64, t13980: f64, t13985: f64, t13995: f64, t14069: f64, t17637: f64, t17670: f64, t17681: f64, t17714: f64, t3117: f64, t43385: f64, t4582: f64, t50084: f64, t50094: f64, t50098: f64, t50100: f64, t50110: f64, t50113: f64, t50116: f64) -> f64 {
    let t62556 = t2960 * t17624;
    let t62559 = t973 * t698 * t5884;
    let t62565 = t973 * t698 * t5889;
    let t62576 = -t50084 / 1728.0_f64 - t10876 * t4582 * t17670 * t13980 / 512.0_f64 - 3.0_f64 / 256.0_f64 * t43385 * t4582 * t17670 * t13985 - t3117 * t17637 / 1152.0_f64 + t62556 / 81.0_f64 + t62559 / 648.0_f64 + t10949 * t17714 / 768.0_f64 + t50094 / 1728.0_f64 - t62565 / 1296.0_f64 + 11.0_f64 / 486.0_f64 * t50098 + t13995 * t14069 / 1152.0_f64 - t10937 * t17681 / 432.0_f64 + t50100 / 216.0_f64 + t50110 / 162.0_f64 + t50113 / 324.0_f64 + 7.0_f64 / 972.0_f64 * t50116;
    t62576
}
