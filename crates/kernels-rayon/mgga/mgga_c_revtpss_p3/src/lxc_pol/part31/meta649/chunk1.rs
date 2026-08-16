//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2140/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2140(t1583: f64, t4343: f64, t25207: f64, t106590: f64, t106593: f64, t106596: f64, t106602: f64, t106606: f64, t106611: f64, t106618: f64, t18280: f64, t1940: f64, t1963: f64, t2403: f64, t25206: f64, t25445: f64, t27169: f64, t27368: f64, t27376: f64, t27382: f64, t27385: f64, t27387: f64, t29599: f64, t29705: f64, t5824: f64, t7010: f64, t7087: f64, t7091: f64, t7783: f64, t92819: f64, t98637: f64) -> (f64, f64) {
    let t106625 = t4343 * t1583;
    let t106626 = t25207 * t106625;
    let t106636 = 2.0_f64 * t27382 * t106590 + t1940 * t25445 * t106593 + 2.0_f64 * t106596 * t27385 + 3.0_f64 / 2.0_f64 * t2403 * t29705 * t7010 - t1940 * t7091 * t106602 / 2.0_f64 - t1940 * t7091 * t106606 / 2.0_f64 + t27382 * t106611 + t1940 * t1963 * t18280 / 2.0_f64 - 3.0_f64 * t92819 * t29599 + 3.0_f64 / 2.0_f64 * t2403 * t1963 * t106618 + 3.0_f64 * t2403 * t7783 * t27169 - 3.0_f64 * t25206 * t106626 - t1940 * t27368 * t27387 - 3.0_f64 * t98637 * t27376 + t1940 * t7087 * t5824 / 2.0_f64;
    (t106625, t106636)
}
