//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 501/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk501(t123: f64, t6514: f64, t2326: f64, t9074: f64, t4261: f64, t6510: f64, t447: f64, t9198: f64, t1064: f64, t2293: f64, t2344: f64, t2343: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9204 = t6514 * t123;
    let t9205 = t9204 * t2326;
    let t9207 = 0.71137516589190373998e-2_f64 * t9074 * t9205;
    let t9208 = t4261 * t6510;
    let t9210 = 0.47425011059460249332e-2_f64 * t9074 * t9208;
    let t9211 = t9198 * t447;
    let t9212 = t1064 * t9211;
    let t9215 = t2344 * t2293;
    let t9216 = t2343 * t9215;
    (t9204, t9207, t9210, t9211, t9212, t9215, t9216)
}
