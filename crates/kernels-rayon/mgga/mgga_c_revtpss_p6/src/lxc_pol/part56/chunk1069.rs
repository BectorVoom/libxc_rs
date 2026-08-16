//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1069/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1069(t124571: f64, t33494: f64, t487: f64, t1243: f64, t1259: f64, t1276: f64, t33461: f64, t33469: f64, t33474: f64, t33462: f64, t33477: f64, t33517: f64, t8937: f64, t96881: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t124573 = t124571 * t487 * t33494;
    let t124577 = t1276 * t1243 * t1259;
    let t124578 = t33461 * t124577;
    let t124584 = t33469 * t124577;
    let t124590 = t33474 * t33494;
    let t124594 = t33477 * t33462 * t1259;
    let t124601 = t8937 * t96881 * t33517;
    (t124573, t124577, t124578, t124584, t124590, t124594, t124601)
}
