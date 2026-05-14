//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 856/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk856<F: Float>(t6205: F, t973: F, t3014: F, t6189: F, t1622: F, t1634: F, t2943: F, t2968: F, t2987: F, t3012: F, t311: F, t4647: F, t4685: F, t6106: F, t6108: F, t6112: F, t6144: F, t6147: F, t6152: F, t6158: F, t6174: F, t6177: F, t6185: F, t6190: F, t946: F, t965: F) -> (F, F, F) {
    let t6206 = t6205 * t973;
    let t6209 = t6189 * t3014;
    let t6212 = -0.310907e-1 * t6152 * t311 + 2.0 * t4647 * t1622 - 2.0 * t2943 * t6158 + 1.0 * t946 * t6174 + 0.32163958997385070134e2 * t2968 * t6177 + t6106 - t6108 + t6112 - t6144 - t6147 - 0.19751673498613801407e-1 * t6185 + 0.11696447245269292414e1 * t4685 * t1634 - 0.11696447245269292414e1 * t2987 * t6190 + 0.5848223622634646207e0 * t965 * t6206 + 0.17315859105681463759e2 * t3012 * t6209;
    (t6206, t6209, t6212)
}
