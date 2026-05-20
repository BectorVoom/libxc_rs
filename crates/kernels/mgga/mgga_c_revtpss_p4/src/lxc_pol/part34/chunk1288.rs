//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1288/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1288<F: Float>(t1544: F, t6079: F, t27383: F, t23429: F, t30: F, t1468: F, t5966: F, t5824: F, t1583: F, t106516: F, t1940: F, t1963: F, t22670: F, t2403: F, t25206: F, t25445: F, t27368: F, t29591: F, t29606: F, t29705: F, t29713: F, t29719: F, t4541: F, t7091: F, t7749: F, t7783: F, t7787: F, t92742: F, t98722: F) -> (F, F) {
    let t113440 = t1544 * t6079;
    let t113441 = t27383 * t113440;
    let t113444 = t30 * t23429;
    let t113454 = t1468 * t5966;
    let t113461 = t5824 * t1544;
    let t113465 = t1468 * t6079;
    let t113484 = t5824 * t1583;
    let t113491 = F::new(9.0) * t25206 * t113441 - F::new(3.0) * t1940 * t92742 * t113444 + F::new(9.0) / F::new(2.0) * t2403 * t7783 * t29606 + F::new(3.0) / F::new(2.0) * t1940 * t7783 * t5824 + F::new(9.0) * t4541 * t1963 * t113454 + F::new(9.0) / F::new(2.0) * t2403 * t29705 * t7749 + F::new(9.0) / F::new(2.0) * t2403 * t1963 * t113461 + F::new(3.0) * t1940 * t25445 * t113465 + F::new(3.0) / F::new(2.0) * t1940 * t29705 * t1468 + F::new(9.0) * t4541 * t7783 * t29591 + F::new(3.0) * t1940 * t98722 * t29713 - F::new(3.0) / F::new(2.0) * t1940 * t27368 * t29719 + t1940 * t1963 * t22670 / F::new(2.0) - F::new(3.0) / F::new(2.0) * t1940 * t7091 * t113484 - F::new(3.0) / F::new(2.0) * t1940 * t106516 * t7787;
    (t113440, t113491)
}
