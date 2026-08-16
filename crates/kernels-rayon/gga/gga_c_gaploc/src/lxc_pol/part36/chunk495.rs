//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 495/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk495(t123: f64, t9078: f64, t4385: f64, t1365: f64, t6520: f64, t6525: f64, t6417: f64, t883: f64, t2325: f64, t882: f64, t2321: f64, t2440: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9079 = t9078 * t123;
    let t9080 = t9079 * t4385;
    let t9083 = t1365 * t6520;
    let t9085 = 0.23712505529730124666e-2_f64 * t6525 * t9083;
    let t9086 = t883 * t6417;
    let t9087 = t2325 * t9086;
    let t9089 = 0.23712505529730124666e-2_f64 * t882 * t9087;
    let t9090 = t2440 * t2321;
    (t9079, t9080, t9085, t9086, t9089, t9090)
}
