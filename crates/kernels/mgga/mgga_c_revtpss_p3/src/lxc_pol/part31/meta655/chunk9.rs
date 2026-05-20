//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2204/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2204<F: Float>(t108187: F, t25878: F, t6861: F, t7274: F, t30081: F, t689: F, t94768: F, t94763: F, t5722: F, t97783: F, t2022: F, t22252: F, t25921: F, t30057: F, t30089: F, t543: F, t7292: F, t7295: F, t7301: F, t94876: F, t98101: F, t98104: F, t98305: F, t98310: F, t98312: F, t98314: F) -> (F, F) {
    let t108474 = t25878 * t108187;
    let t108484 = t7274 * t6861;
    let t108493 = t30081 * t689;
    let t108494 = t94768 * t108493;
    let t108496 = t94763 * t108493;
    let t108498 = t97783 * t5722;
    let t108500 = -F::cast_from(0.34270468708064099208e-1_f64) * t98101 - F::cast_from(0.22849835011101738147e-2_f64) * t94876 + F::cast_from(0.51405703062096148813e-1_f64) * t108474 - F::cast_from(0.19274729307122665472e-1_f64) * t98104 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t2022 * t22252 * t543 + F::cast_from(0.4336814094102599731e0_f64) * t25921 * t30089 + F::cast_from(0.4336814094102599731e0_f64) * t7295 * t7301 * t108484 * t543 - t98305 - F::cast_from(0.4336814094102599731e0_f64) * t7292 * t30057 + t98310 + F::cast_from(0.45699670022203476294e-2_f64) * t98312 - F::cast_from(0.3427046870806409921e-2_f64) * t98314 + F::cast_from(0.14456046980341999104e-1_f64) * t108494 - F::cast_from(0.25702851531048074406e-1_f64) * t108496 - F::cast_from(0.19514881078765566037e-1_f64) * t108498;
    (t108484, t108500)
}
