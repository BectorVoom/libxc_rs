//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1160/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1160<F: Float>(t106516: F, t113096: F, t113103: F, t113107: F, t113415: F, t113432: F, t113440: F, t1544: F, t1583: F, t1940: F, t1962: F, t1963: F, t198: F, t207: F, t23114: F, t23148: F, t23279: F, t23421: F, t23429: F, t2403: F, t25445: F, t27368: F, t29598: F, t29705: F, t4541: F, t5962: F, t5966: F, t6075: F, t6079: F, t7091: F, t7783: F, t892: F, t92742: F, t98722: F) -> (F,) {
    let t114089 = -3.0 * t1940 * t106516 * t1583 + 18.0 * t2403 * t25445 * t113440 + 3.0 * t2403 * t1963 * t23148 - 3.0 * t1940 * t27368 * t6075 - t1940 * t7091 * t23421 - 18.0 * t2403 * t27368 * t29598 + 9.0 * t2403 * t29705 * t1544 + 6.0 * t198 * t23114 * t1962 * t892 + 18.0 * t4541 * t7783 * t5966 - 9.0 * t2403 * t7091 * t113432 - 9.0 * t2403 * t7091 * t113103 + 6.0 * t1940 * t25445 * t113107 + 6.0 * t1940 * t98722 * t6079 + 9.0 * t2403 * t7783 * t5962 - 6.0 * t1940 * t92742 * t23429 + t198 * t207 * t113415 * t892 - 18.0 * t4541 * t7091 * t113096 + 18.0 * t4541 * t1963 * t23279;
    (t114089,)
}
