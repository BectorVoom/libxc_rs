//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1208/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1208(t2374: f64, t2409: f64, t7669: f64, t7672: f64, t10694: f64, t2466: f64, t2493: f64, t2494: f64, t2495: f64, t24950: f64, t24955: f64, t24957: f64, t24960: f64, t24964: f64, t2512: f64, t2518: f64, t2520: f64, t2521: f64, t2530: f64, t2531: f64, t2537: f64, t2538: f64, t7727: f64, t7731: f64, t7738: f64, t7753: f64, t7759: f64, t7793: f64, t7794: f64, t7799: f64, t7801: f64, t7810: f64, t7813: f64, t7817: f64, t7825: f64, t7828: f64, t817: f64, t836: f64) -> (f64, f64) {
    let t24968 = 0.3103500882342370105e4_f64 * t7669 * t2374 * t7672 * t2409;
    let t24972 = 0.38597619813444837568e3_f64 * t7825 * t7738 + 0.21053604230838734656e2_f64 * t2537 * t2531 * t2466 + 0.61523382126046769581e4_f64 * t7753 * t10694 * t2466 - 0.11579285944033451271e4_f64 * t7759 * t2521 * t2512 - 8.0_f64 * t2493 * t7794 * t817 + 0.1286587327114827919e3_f64 * t2518 * t7793 * t2520 * t817 + 0.12414802127193579148e5_f64 * t7799 * t2494 * t7801 * t2512 - 0.14035736153892489771e2_f64 * t7828 * t7727 + 0.2077890707925103596e3_f64 * t7810 * t7731 - 0.62336721237753107879e3_f64 * t7813 * t2538 * t2466 - 0.46785787179641632568e1_f64 * t2530 * t7817 * t836 + 0.69263023597503453196e2_f64 * t2537 * t24950 * t836 + t24955 - t24957 + t24960 - t24964 - t24968 + 36.0_f64 * t2518 * t2495 * t2512;
    (t24968, t24972)
}
