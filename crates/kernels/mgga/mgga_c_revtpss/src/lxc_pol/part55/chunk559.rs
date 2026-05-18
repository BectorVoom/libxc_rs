//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 559/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk559<F: Float>(t4707: F, t973: F, t1633: F, t3014: F, t972: F, t1622: F, t1634: F, t2938: F, t2943: F, t2968: F, t2982: F, t2987: F, t3012: F, t311: F, t4589: F, t4592: F, t4594: F, t4597: F, t4634: F, t4638: F, t4644: F, t4647: F, t4652: F, t4670: F, t4674: F, t4683: F, t4685: F, t4690: F, t946: F, t955: F, t965: F, t974: F) -> F {
    let t4708 = t4707 * t973;
    let t4711 = t1633 * t3014;
    let t4712 = t4711 * t972;
    let t4715 = -F::new(0.310907e-1) * t4644 * t311 + F::new(1.0) * t4647 * t955 + F::new(1.0) * t2938 * t1622 - F::new(2.0) * t2943 * t4652 + F::new(1.0) * t946 * t4670 + F::new(0.32163958997385070134e2) * t2968 * t4674 + t4589 - t4592 - t4594 + t4597 - t4634 - t4638 - F::new(0.19751673498613801407e-1) * t4683 + F::new(0.5848223622634646207e0) * t4685 * t974 + F::new(0.5848223622634646207e0) * t2982 * t1634 - F::new(0.11696447245269292414e1) * t2987 * t4690 + F::new(0.5848223622634646207e0) * t965 * t4708 + F::new(0.17315859105681463759e2) * t3012 * t4712;
    t4715
}
