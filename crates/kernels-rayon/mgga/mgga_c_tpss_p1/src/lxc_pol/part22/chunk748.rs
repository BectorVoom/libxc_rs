//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 748/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk748(t3882: f64, t904: f64, t1448: f64, t2621: f64, t903: f64, t1437: f64, t1449: f64, t2545: f64, t2550: f64, t2575: f64, t2589: f64, t2594: f64, t2619: f64, t305: f64, t3764: f64, t3767: f64, t3769: f64, t3772: f64, t3809: f64, t3813: f64, t3819: f64, t3822: f64, t3827: f64, t3845: f64, t3849: f64, t3858: f64, t3860: f64, t3865: f64, t877: f64, t886: f64, t896: f64, t905: f64) -> (f64, f64, f64, f64) {
    let t3883 = t3882 * t904;
    let t3886 = t1448 * t2621;
    let t3887 = t3886 * t903;
    let t3890 = -0.310907e-1_f64 * t3819 * t305 + 1.0_f64 * t3822 * t886 + 1.0_f64 * t2545 * t1437 - 2.0_f64 * t2550 * t3827 + 1.0_f64 * t877 * t3845 + 0.32163958997385070134e2_f64 * t2575 * t3849 + t3764 - t3767 - t3769 + t3772 - t3809 - t3813 - 0.19751673498613801407e-1_f64 * t3858 + 0.5848223622634646207e0_f64 * t3860 * t905 + 0.5848223622634646207e0_f64 * t2589 * t1449 - 0.11696447245269292414e1_f64 * t2594 * t3865 + 0.5848223622634646207e0_f64 * t896 * t3883 + 0.17315859105681463759e2_f64 * t2619 * t3887;
    (t3883, t3886, t3887, t3890)
}
