//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2148/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2148<F: Float>(t1583: F, t4343: F, t25207: F, t106590: F, t106593: F, t106596: F, t106602: F, t106606: F, t106611: F, t106618: F, t18280: F, t1940: F, t1963: F, t2403: F, t25206: F, t25445: F, t27169: F, t27368: F, t27376: F, t27382: F, t27385: F, t27387: F, t29599: F, t29705: F, t5824: F, t7010: F, t7087: F, t7091: F, t7783: F, t92819: F, t98637: F) -> (F, F) {
    let t106625 = t4343 * t1583;
    let t106626 = t25207 * t106625;
    let t106636 = F::new(2.0) * t27382 * t106590 + t1940 * t25445 * t106593 + F::new(2.0) * t106596 * t27385 + F::new(3.0) / F::new(2.0) * t2403 * t29705 * t7010 - t1940 * t7091 * t106602 / F::new(2.0) - t1940 * t7091 * t106606 / F::new(2.0) + t27382 * t106611 + t1940 * t1963 * t18280 / F::new(2.0) - F::new(3.0) * t92819 * t29599 + F::new(3.0) / F::new(2.0) * t2403 * t1963 * t106618 + F::new(3.0) * t2403 * t7783 * t27169 - F::new(3.0) * t25206 * t106626 - t1940 * t27368 * t27387 - F::new(3.0) * t98637 * t27376 + t1940 * t7087 * t5824 / F::new(2.0);
    (t106625, t106636)
}
