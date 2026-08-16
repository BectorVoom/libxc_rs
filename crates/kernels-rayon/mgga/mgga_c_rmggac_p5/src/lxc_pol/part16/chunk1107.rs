//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1107/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1107(t43752: f64, t43757: f64, t43761: f64, t43763: f64, t47345: f64, t47347: f64, t47349: f64, t47351: f64, t47353: f64, t47355: f64, t47357: f64, t47359: f64, t47361: f64, t47365: f64, t47367: f64, t47371: f64, t47375: f64, t4985: f64, t9624: f64) -> f64 {
    let t49006 = -t43752 + 0.1702583995731913576e-4_f64 * t47345 - 0.5107751987195740728e-4_f64 * t47347 + 0.5107751987195740728e-4_f64 * t47349 + 0.3405167991463827152e-4_f64 * t47351 - 0.1702583995731913576e-4_f64 * t47353 + 0.10215503974391481456e-3_f64 * t47355 - 0.15323255961587222184e-3_f64 * t47357 - 0.11918087970123395032e-3_f64 * t47359 - 0.68186654135613354325e-2_f64 * t47361 - 0.68186654135613354325e-2_f64 * t47365 + 0.20455996240684006298e-1_f64 * t47367 - t43757 - t43761 - t43763 - 0.5987120850931904282e-1_f64 * t47371 + 0.39726959900411316773e-4_f64 * t47375 - 0.23948483403727617128e0_f64 * t4985 * t9624;
    t49006
}
