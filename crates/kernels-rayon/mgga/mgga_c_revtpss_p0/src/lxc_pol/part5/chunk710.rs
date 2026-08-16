//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 710/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk710(t4707: f64, t973: f64, t1633: f64, t3014: f64, t972: f64, t1622: f64, t1634: f64, t2938: f64, t2943: f64, t2968: f64, t2982: f64, t2987: f64, t3012: f64, t311: f64, t4589: f64, t4592: f64, t4594: f64, t4597: f64, t4634: f64, t4638: f64, t4644: f64, t4647: f64, t4652: f64, t4670: f64, t4674: f64, t4683: f64, t4685: f64, t4690: f64, t946: f64, t955: f64, t965: f64, t974: f64) -> (f64, f64, f64, f64) {
    let t4708 = t4707 * t973;
    let t4711 = t1633 * t3014;
    let t4712 = t4711 * t972;
    let t4715 = -0.310907e-1_f64 * t4644 * t311 + 1.0_f64 * t4647 * t955 + 1.0_f64 * t2938 * t1622 - 2.0_f64 * t2943 * t4652 + 1.0_f64 * t946 * t4670 + 0.32163958997385070134e2_f64 * t2968 * t4674 + t4589 - t4592 - t4594 + t4597 - t4634 - t4638 - 0.19751673498613801407e-1_f64 * t4683 + 0.5848223622634646207e0_f64 * t4685 * t974 + 0.5848223622634646207e0_f64 * t2982 * t1634 - 0.11696447245269292414e1_f64 * t2987 * t4690 + 0.5848223622634646207e0_f64 * t965 * t4708 + 0.17315859105681463759e2_f64 * t3012 * t4712;
    (t4708, t4711, t4712, t4715)
}
