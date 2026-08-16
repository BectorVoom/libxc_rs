//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 593/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk593(t2595: f64, t2621: f64, t2475: f64, t2478: f64, t2485: f64, t2528: f64, t2536: f64, t2542: f64, t2545: f64, t2550: f64, t2552: f64, t2570: f64, t2575: f64, t2578: f64, t2587: f64, t2589: f64, t2594: f64, t2596: f64, t2614: f64, t2619: f64, t305: f64, t877: f64, t886: f64, t896: f64, t905: f64) -> (f64, f64) {
    let t2622 = t2595 * t2621;
    let t2625 = -0.310907e-1_f64 * t2542 * t305 + 2.0_f64 * t2545 * t886 - 2.0_f64 * t2550 * t2552 + 1.0_f64 * t877 * t2570 + 0.32163958997385070134e2_f64 * t2575 * t2578 + t2475 - t2478 + t2485 - t2528 - t2536 - 0.19751673498613801407e-1_f64 * t2587 + 0.11696447245269292414e1_f64 * t2589 * t905 - 0.11696447245269292414e1_f64 * t2594 * t2596 + 0.5848223622634646207e0_f64 * t896 * t2614 + 0.17315859105681463759e2_f64 * t2619 * t2622;
    (t2622, t2625)
}
