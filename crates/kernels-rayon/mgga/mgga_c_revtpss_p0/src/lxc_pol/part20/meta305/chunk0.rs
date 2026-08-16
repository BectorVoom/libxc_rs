//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1194/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1194(t1168: f64, t3471: f64, t3479: f64, t1156: f64, t3451: f64, t1169: f64, t12430: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64, t12329: f64, t12332: f64) -> (f64, f64, f64, f64) {
    let t12508 = t3471 * t3479 * t1168;
    let t12511 = t1156 * t3451;
    let t12514 = t12430 * t1169;
    let t12531 = 0.5519e-1_f64 * t12252 + 0.36793333333333333333e-1_f64 * t12259 + 0.27595e0_f64 * t12261 - 0.16557e0_f64 * t12263 - 0.33114e0_f64 * t12265 - 0.16557e0_f64 * t12271 + 0.49671e0_f64 * t12275 + 0.82785e-1_f64 * t12279 - 0.82785e-1_f64 * t12284 + 0.49671e0_f64 * t12289 - 0.60384999999999999999e0_f64 * t12292 + 0.258925e1_f64 * t12323 + 0.19419375e1_f64 * t12329 - 0.412621875e-1_f64 * t12332;
    (t12508, t12511, t12514, t12531)
}
