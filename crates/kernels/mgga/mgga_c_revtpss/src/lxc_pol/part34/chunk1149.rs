//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1149/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1149<F: Float>(t100321: F, t106934: F, t106938: F, t106960: F, t107048: F, t107072: F, t1665: F, t1671: F, t23503: F, t23630: F, t23823: F, t23892: F, t23921: F, t23976: F, t24013: F, t24024: F, t24034: F, t25517: F, t25522: F, t27450: F, t27479: F, t27489: F, t27526: F, t27527: F, t6278: F, t6302: F, t6327: F, t6331: F, t6339: F, t7117: F, t7122: F, t7132: F, t93548: F, t93611: F, t93725: F) -> (F,) {
    let t113600 = 0.28582678745379824648e-3 * t7132 * t23976 + 0.14291339372689912324e-2 * t27489 * t6327 + 0.57165357490759649295e-3 * t106934 + 0.95275595817932748825e-3 * t106938 + 0.12862205435420921092e-2 * t27450 * t6302 + 0.42874018118069736972e-3 * t7122 * t23823 + 0.12862205435420921092e-2 * t107048 * t1671 - 0.85748036236139473944e-3 * t25522 * t23892 + 0.12862205435420921092e-2 * t93548 * t24013 + t93611 + 0.11433071498151929859e-2 * t106960 - 0.17149607247227894789e-2 * t25517 * t23921 + 0.17149607247227894789e-2 * t7132 * t23630 - 0.17149607247227894789e-2 * t27489 * t6331 - 0.25724410870841842183e-2 * t93725 * t24034 - 0.12862205435420921092e-2 * t107072 * t1665 - 0.12862205435420921092e-2 * t27479 * t6278 - 0.42874018118069736972e-3 * t7117 * t24024 + 0.25724410870841842183e-2 * t100321 * t6339 - t27526 * t27527 * t23503 / 48.0;
    (t113600,)
}
