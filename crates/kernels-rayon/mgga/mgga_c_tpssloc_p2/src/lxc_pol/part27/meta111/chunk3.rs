//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 684/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk684(t2369: f64, t2512: f64, t118: f64, t168: f64, t181: f64, t2393: f64, t2408: f64, t2417: f64, t2423: f64, t2426: f64, t2454: f64, t2460: f64, t2462: f64, t2472: f64, t2477: f64, t2480: f64, t2486: f64, t2490: f64, t2494: f64, t2495: f64, t2505: f64, t2510: f64, t268: f64, t725: f64, t732: f64, t740: f64, t747: f64) -> (f64, f64) {
    let t2513 = t2369 * t2512;
    let t2516 = -0.70983522622222222221e-3_f64 * t118 * t2393 * t168 - 0.34246666666666666666e-1_f64 * t268 * t2454 * t732 - 2.0_f64 * t2460 * t2462 + 1.0_f64 * t725 * t2472 + 0.32163958997385070134e2_f64 * t2477 * t2480 + t2426 + t2486 + t2423 - t2408 - t2417 - 0.24415263074675393405e-3_f64 * t118 * t2393 * t181 - 0.10843581300301739842e-1_f64 * t268 * t2490 * t747 - 0.11696447245269292414e1_f64 * t2494 * t2495 + 0.5848223622634646207e0_f64 * t740 * t2505 + 0.17315859105681463759e2_f64 * t2510 * t2513;
    (t2513, t2516)
}
