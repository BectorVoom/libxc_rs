//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 986/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk986(t77312: f64, t2039: f64, t2475: f64, t270: f64, t638: f64, t2046: f64, t2050: f64, t31: f64, t71214: f64, t71222: f64, t14444: f64, t1632: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t77313 = 0.21684485328539747656e-4_f64 * t77312;
    let t77316 = t638 * t2039 * t2475 * t270;
    let t77317 = 0.15243824895787514157e-3_f64 * t77316;
    let t77320 = t2046 * t2050 * t2475 * t31;
    let t77321 = 0.21684485328539747656e-4_f64 * t77320;
    let t77322 = 0.15243824895787514157e-3_f64 * t71214;
    let t77323 = 0.21684485328539747656e-4_f64 * t71222;
    let t77327 = t14444 * t1632;
    (t77313, t77317, t77321, t77322, t77323, t77327)
}
