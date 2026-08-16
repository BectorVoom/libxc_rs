//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 964/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk964(t11905: f64, t2376: f64, t2868: f64, t40063: f64, t40076: f64, t45938: f64, t45942: f64, t45947: f64, t45949: f64, t45951: f64, t45956: f64, t45960: f64, t45964: f64, t45966: f64, t45974: f64, t45976: f64, t45982: f64, t6557: f64, t7567: f64, t884: f64, t9025: f64) -> f64 {
    let t45989 = 0.25538759935978703638e-4_f64 * t45938 - 0.25538759935978703638e-4_f64 * t45942 - 0.23942587439980034662e-4_f64 * t45947 + 0.25538759935978703638e-4_f64 * t45949 - 0.76616279807936110914e-4_f64 * t45951 - 0.25538759935978703638e-4_f64 * t45956 + 0.76616279807936110914e-4_f64 * t45960 - 0.10215503974391481455e-3_f64 * t45964 - 0.85129199786595678796e-5_f64 * t45966 - 0.11974241701863808564e0_f64 * t11905 * t2376 - t40063 - t40076 - 0.12769379967989351819e-4_f64 * t45974 + 0.25538759935978703638e-4_f64 * t45976 + 0.12769379967989351819e-4_f64 * t45982 - 0.23948483403727617128e0_f64 * t884 * t7567 * t6557 - 0.11974241701863808564e0_f64 * t2868 * t9025;
    t45989
}
