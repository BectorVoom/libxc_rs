//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 729/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk729<F: Float>(t4707: F, t973: F, t1633: F, t3014: F, t972: F, t1622: F, t1634: F, t2938: F, t2943: F, t2968: F, t2982: F, t2987: F, t3012: F, t311: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4644: F, t4647: F, t4652: F, t4670: F, t4674: F, t4683: F, t4685: F, t4690: F, t946: F, t955: F, t965: F, t974: F) -> (F, F, F, F) {
    let t4708 = t4707 * t973;
    let t4711 = t1633 * t3014;
    let t4712 = t4711 * t972;
    let t4715 = -F::cast_from(0.310907e-1_f64) * t4644 * t311 + F::cast_from(1.0_f64) * t4647 * t955 + F::cast_from(1.0_f64) * t2938 * t1622 - F::cast_from(2.0_f64) * t2943 * t4652 + F::cast_from(1.0_f64) * t946 * t4670 + F::cast_from(0.32163958997385070134e2_f64) * t2968 * t4674 + t4589 - t4592 - t4594 + t4597 - t4634 - t4638 - F::cast_from(0.19751673498613801407e-1_f64) * t4683 + F::cast_from(0.5848223622634646207e0_f64) * t4685 * t974 + F::cast_from(0.5848223622634646207e0_f64) * t2982 * t1634 - F::cast_from(0.11696447245269292414e1_f64) * t2987 * t4690 + F::cast_from(0.5848223622634646207e0_f64) * t965 * t4708 + F::cast_from(0.17315859105681463759e2_f64) * t3012 * t4712;
    (t4708, t4711, t4712, t4715)
}
