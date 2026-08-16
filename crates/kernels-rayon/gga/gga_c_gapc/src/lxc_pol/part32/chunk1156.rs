//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 1156/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk1156(t34235: f64, t34238: f64, t34241: f64, t34245: f64, t34249: f64, t34252: f64, t34255: f64, t34258: f64, t34264: f64, t34269: f64, t34274: f64, t2967: f64, t3179: f64, t4915: f64) -> (f64, f64) {
    let t34276 = 0.51491428373437201895e-6_f64 * t34235 + 0.20010856351627032588e-8_f64 * t34238 + 0.17376185052903442709e-3_f64 * t34241 + 0.24581606547037760418e-8_f64 * t34245 - 0.81938688490125868062e-9_f64 * t34249 - 0.51491428373437201896e-5_f64 * t34252 - 0.16387737698025173612e-8_f64 * t34255 + 0.11049275749843950005e-7_f64 * t34258 + 0.66295654499063700028e-7_f64 * t34264 - 0.54785992259642918774e-7_f64 * t34269 + 0.39291224566445086216e-8_f64 * t34274;
    let t34285 = 24.0_f64 * t4915 * t2967 * t3179;
    (t34276, t34285)
}
