//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1282/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1282(t10578: f64, t9575: f64, t9572: f64, t2434: f64, t2496: f64, t2629: f64, t676: f64, t9419: f64, t9866: f64, t123: f64, t2390: f64, t2630: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t39423 = t10578 * t9575;
    let t39424 = 0.86748650402413918736e-1_f64 * t39423;
    let t39425 = t10578 * t9572;
    let t39426 = 0.1301229756036208781e0_f64 * t39425;
    let t39427 = t2434 * t2496;
    let t39429 = 0.12842595503380418954e1_f64 * t2629 * t39427;
    let t39430 = t676 * t9419;
    let t39432 = 0.38527786510141256862e1_f64 * t2629 * t39430;
    let t39433 = t10578 * t9866;
    let t39434 = 0.19263893255070628431e1_f64 * t39433;
    let t39436 = t2390 * t123 * t2630;
    (t39424, t39426, t39427, t39429, t39430, t39432, t39434, t39436)
}
