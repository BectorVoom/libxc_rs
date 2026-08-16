//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1165/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1165(t1843: f64, t39149: f64, t7064: f64, t12255: f64, t2508: f64, t2586: f64, t43244: f64, t43248: f64, t43254: f64, t43257: f64, t43260: f64, t43263: f64, t43265: f64, t43267: f64, t43269: f64) -> f64 {
    let t47731 = t7064 * t1843 * t39149;
    let t47734 = t2508 * t12255 * t2586;
    let t47736 = -0.23071578690426672851e-1_f64 * t43244 - 0.23071578690426672851e-1_f64 * t43248 + t43254 + t43257 + 0.32043859292259267849e-3_f64 * t47731 + t43260 + t43263 + t43265 - t43267 - t43269 - 0.23071578690426672851e-1_f64 * t47734;
    t47736
}
