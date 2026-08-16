//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1045/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1045(t11347: f64, t885: f64, t3857: f64, t895: f64, t1441: f64, t2618: f64, t10963: f64, t11286: f64, t11289: f64, t11294: f64, t1437: f64, t1449: f64, t2545: f64, t2570: f64, t2578: f64, t2614: f64, t2622: f64, t305: f64, t3822: f64, t3845: f64, t3860: f64, t877: f64, t8837: f64, t886: f64, t8894: f64, t905: f64) -> f64 {
    let t11348 = t11347 * t885;
    let t11351 = t3857 * t895;
    let t11356 = t1441 * t2618;
    let t11361 = -0.310907e-1_f64 * t11286 * t305 + 2.0_f64 * t11289 * t886 + 1.0_f64 * t3822 * t2570 + 0.32163958997385070134e2_f64 * t11294 * t2578 + 1.0_f64 * t8837 * t1437 + 2.0_f64 * t2545 * t3845 + 1.0_f64 * t877 * t11348 + 0.11696447245269292414e1_f64 * t11351 * t905 + 0.5848223622634646207e0_f64 * t3860 * t2614 + 0.17315859105681463759e2_f64 * t11356 * t2622 + 0.5848223622634646207e0_f64 * t8894 * t1449 - t10963;
    t11361
}
