//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 812/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk812(t44308: f64, t35900: f64, t883: f64, t2761: f64, t9074: f64, t1365: f64, t36211: f64, t6525: f64, t35888: f64, t35893: f64, t4261: f64, t11280: f64, t2326: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44309 = 0.11856252764865062333e-2_f64 * t44308;
    let t44310 = t883 * t35900;
    let t44312 = t9074 * t2761 * t44310;
    let t44313 = 0.23712505529730124666e-2_f64 * t44312;
    let t44315 = t6525 * t1365 * t36211;
    let t44316 = 0.11856252764865062333e-2_f64 * t44315;
    let t44318 = t9074 * t1365 * t35888;
    let t44319 = 0.35568758294595186999e-2_f64 * t44318;
    let t44321 = t9074 * t4261 * t35893;
    let t44322 = 0.23712505529730124666e-2_f64 * t44321;
    let t44324 = t9074 * t11280 * t2326;
    (t44309, t44310, t44313, t44316, t44319, t44322, t44324)
}
