//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 486/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk486(t2321: f64, t6603: f64, t9074: f64, t1365: f64, t6520: f64, t6525: f64, t6417: f64, t883: f64, t2325: f64, t882: f64, t2440: f64, t2312: f64, t3130: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9075 = t6603 * t2321;
    let t9077 = 0.23712505529730124666e-2_f64 * t9074 * t9075;
    let t9083 = t1365 * t6520;
    let t9085 = 0.23712505529730124666e-2_f64 * t6525 * t9083;
    let t9086 = t883 * t6417;
    let t9087 = t2325 * t9086;
    let t9089 = 0.23712505529730124666e-2_f64 * t882 * t9087;
    let t9090 = t2440 * t2321;
    let t9092 = 0.23712505529730124666e-2_f64 * t882 * t9090;
    let t9094 = 0.23712505529730124666e-2_f64 * t2312 * t3130;
    (t9077, t9085, t9086, t9089, t9092, t9094)
}
