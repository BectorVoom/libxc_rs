//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 847/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk847(t2826: f64, t4338: f64, t136: f64, t4343: f64, t908: f64, t4347: f64, t2766: f64, t2810: f64, t2823: f64, t2824: f64, t4335: f64, t4340: f64, t4345: f64, t4349: f64, t4363: f64, t4371: f64, t4379: f64, t4381: f64, t4384: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4386 = t2826 * t4338;
    let t4387 = t136 * t4386;
    let t4389 = t908 * t4343;
    let t4390 = t136 * t4389;
    let t4392 = t908 * t4347;
    let t4393 = t136 * t4392;
    let t4395 = -0.9494625e0_f64 * t4363 + 0.1898925e1_f64 * t4371 + t2810 + 0.99655555555555555557e-1_f64 * t2766 + 0.99655555555555555557e-1_f64 * t4335 - 0.19931111111111111111e0_f64 * t4340 + 0.59793333333333333334e0_f64 * t4345 - 0.29896666666666666667e0_f64 * t4349 + 0.15358125e0_f64 * t4379 + 0.3071625e0_f64 * t4381 + t2823 + 0.54771111111111111111e-1_f64 * t2824 + 0.54771111111111111111e-1_f64 * t4384 - 0.27385555555555555556e-1_f64 * t4387 + 0.16431333333333333333e0_f64 * t4390 - 0.82156666666666666667e-1_f64 * t4393;
    (t4386, t4387, t4389, t4390, t4392, t4393, t4395)
}
