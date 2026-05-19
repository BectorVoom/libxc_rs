//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1208/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1208<F: Float>(t2374: F, t2409: F, t7669: F, t7672: F, t10694: F, t2466: F, t2493: F, t2494: F, t2495: F, t24950: F, t24955: F, t24957: F, t24960: F, t24964: F, t2512: F, t2518: F, t2520: F, t2521: F, t2530: F, t2531: F, t2537: F, t2538: F, t7727: F, t7731: F, t7738: F, t7753: F, t7759: F, t7793: F, t7794: F, t7799: F, t7801: F, t7810: F, t7813: F, t7817: F, t7825: F, t7828: F, t817: F, t836: F) -> (F, F) {
    let t24968 = F::cast_from(0.3103500882342370105e4_f64) * t7669 * t2374 * t7672 * t2409;
    let t24972 = F::cast_from(0.38597619813444837568e3_f64) * t7825 * t7738 + F::cast_from(0.21053604230838734656e2_f64) * t2537 * t2531 * t2466 + F::cast_from(0.61523382126046769581e4_f64) * t7753 * t10694 * t2466 - F::cast_from(0.11579285944033451271e4_f64) * t7759 * t2521 * t2512 - F::new(8.0) * t2493 * t7794 * t817 + F::cast_from(0.1286587327114827919e3_f64) * t2518 * t7793 * t2520 * t817 + F::cast_from(0.12414802127193579148e5_f64) * t7799 * t2494 * t7801 * t2512 - F::cast_from(0.14035736153892489771e2_f64) * t7828 * t7727 + F::cast_from(0.2077890707925103596e3_f64) * t7810 * t7731 - F::cast_from(0.62336721237753107879e3_f64) * t7813 * t2538 * t2466 - F::cast_from(0.46785787179641632568e1_f64) * t2530 * t7817 * t836 + F::cast_from(0.69263023597503453196e2_f64) * t2537 * t24950 * t836 + t24955 - t24957 + t24960 - t24964 - t24968 + F::new(36.0) * t2518 * t2495 * t2512;
    (t24968, t24972)
}
