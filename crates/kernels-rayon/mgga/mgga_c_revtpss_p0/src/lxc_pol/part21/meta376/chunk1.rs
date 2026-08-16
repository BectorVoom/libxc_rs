//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1787/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1787(t409: f64, t416: f64, t1134: f64, t3391: f64, t406: f64, t12252: f64, t12259: f64, t12261: f64, t12263: f64, t12265: f64, t12271: f64, t12275: f64, t12279: f64, t12284: f64, t12289: f64, t12292: f64, t12323: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12327 = 1.0_f64 / t409 / t416 / 4.0_f64;
    let t12328 = t3391 * t1134;
    let t12329 = t12327 * t12328;
    let t12331 = 1.0_f64/pow_3_2(t406);
    let t12332 = t12331 * t12328;
    let t12334 = 0.5477111111111111111e-1_f64 * t12252 + 0.36514074074074074075e-1_f64 * t12259 + 0.27385555555555555556e0_f64 * t12261 - 0.16431333333333333333e0_f64 * t12263 - 0.32862666666666666666e0_f64 * t12265 - 0.16431333333333333333e0_f64 * t12271 + 0.49293999999999999999e0_f64 * t12275 + 0.82156666666666666667e-1_f64 * t12279 - 0.82156666666666666668e-1_f64 * t12284 + 0.49293999999999999999e0_f64 * t12289 - 0.59793333333333333333e0_f64 * t12292 + 0.1898925e1_f64 * t12323 + 0.142419375e1_f64 * t12329 - 0.76790625e-1_f64 * t12332;
    (t12327, t12328, t12329, t12331, t12332, t12334)
}
