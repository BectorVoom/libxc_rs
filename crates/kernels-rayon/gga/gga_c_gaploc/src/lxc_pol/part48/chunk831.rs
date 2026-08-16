//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 831/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk831(t42587: f64, t42590: f64, t11182: f64, t2317: f64, t6525: f64, t35900: f64, t883: f64, t2761: f64, t9074: f64, t1365: f64, t36211: f64, t35888: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t44305 = 0.142275033178380748e-1_f64 * t42587;
    let t44306 = 0.142275033178380748e-1_f64 * t42590;
    let t44308 = t6525 * t11182 * t2317;
    let t44309 = 0.11856252764865062333e-2_f64 * t44308;
    let t44310 = t883 * t35900;
    let t44312 = t9074 * t2761 * t44310;
    let t44313 = 0.23712505529730124666e-2_f64 * t44312;
    let t44315 = t6525 * t1365 * t36211;
    let t44316 = 0.11856252764865062333e-2_f64 * t44315;
    let t44318 = t9074 * t1365 * t35888;
    (t44305, t44306, t44309, t44310, t44313, t44316, t44318)
}
