//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1638/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1638(t6079: f64, t1544: f64, t1583: f64, t18850: f64, t1940: f64, t198: f64, t207: f64, t23148: f64, t2403: f64, t40076: f64, t40079: f64, t40194: f64, t40198: f64, t41154: f64, t4541: f64, t4546: f64, t5966: f64, t765: f64, t77357: f64, t77373: f64, t87543: f64, t87676: f64, t87677: f64, t87678: f64, t87679: f64) -> f64 {
    let t87970 = t6079 * t6079;
    let t87987 = -6.0_f64 * t198 * t207 * t41154 * t87970 + 24.0_f64 * t1544 * t2403 * t77373 - 4.0_f64 * t1583 * t1940 * t77357 + 36.0_f64 * t18850 * t4541 * t5966 + 3.0_f64 * t198 * t765 * t87543 + 12.0_f64 * t23148 * t2403 * t4546 + t40076 - t40079 + t40194 + t40198 + t87676 + t87677 - t87678 - t87679;
    t87987
}
