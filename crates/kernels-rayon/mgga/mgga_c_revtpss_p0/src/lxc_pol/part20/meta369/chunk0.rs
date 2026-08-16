//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1344/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1344(t10428: f64, t2414: f64, t10587: f64, t2496: f64, t10467: f64, t705: f64, t707: f64, t190: f64, t39457: f64, t706: f64, t39875: f64, t39894: f64, t9371: f64) -> (f64, f64, f64, f64, f64) {
    let t40155 = 24.0_f64 * t10428 * t2414;
    let t40156 = t10587 * t2496;
    let t40157 = 0.10389515463408878255e3_f64 * t40156;
    let t40158 = t705 * t10467;
    let t40160 = 16.0_f64 * t40158 * t707;
    let t40163 = 4.0_f64 * t706 * t190 * t39457;
    let t40165 = t39894 * t39875 * t9371;
    (t40155, t40157, t40160, t40163, t40165)
}
