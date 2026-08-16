//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2204/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2204(t108187: f64, t25878: f64, t6861: f64, t7274: f64, t30081: f64, t689: f64, t94768: f64, t94763: f64, t5722: f64, t97783: f64, t2022: f64, t22252: f64, t25921: f64, t30057: f64, t30089: f64, t543: f64, t7292: f64, t7295: f64, t7301: f64, t94876: f64, t98101: f64, t98104: f64, t98305: f64, t98310: f64, t98312: f64, t98314: f64) -> (f64, f64) {
    let t108474 = t25878 * t108187;
    let t108484 = t7274 * t6861;
    let t108493 = t30081 * t689;
    let t108494 = t94768 * t108493;
    let t108496 = t94763 * t108493;
    let t108498 = t97783 * t5722;
    let t108500 = -0.34270468708064099208e-1_f64 * t98101 - 0.22849835011101738147e-2_f64 * t94876 + 0.51405703062096148813e-1_f64 * t108474 - 0.19274729307122665472e-1_f64 * t98104 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t2022 * t22252 * t543 + 0.4336814094102599731e0_f64 * t25921 * t30089 + 0.4336814094102599731e0_f64 * t7295 * t7301 * t108484 * t543 - t98305 - 0.4336814094102599731e0_f64 * t7292 * t30057 + t98310 + 0.45699670022203476294e-2_f64 * t98312 - 0.3427046870806409921e-2_f64 * t98314 + 0.14456046980341999104e-1_f64 * t108494 - 0.25702851531048074406e-1_f64 * t108496 - 0.19514881078765566037e-1_f64 * t108498;
    (t108484, t108500)
}
