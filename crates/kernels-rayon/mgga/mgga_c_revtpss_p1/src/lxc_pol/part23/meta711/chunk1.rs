//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2469/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2469(t1432: f64, t5763: f64, t9288: f64, t10069: f64, t14124: f64, t14129: f64, t14231: f64, t10139: f64, t136: f64, t2457: f64, t5659: f64, t14202: f64, t9303: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t47971 = t1432 * t5763 * t9288;
    let t47978 = t10069 * t14124;
    let t47979 = 0.21951497276451705329e-1_f64 * t47978;
    let t47980 = t10069 * t14129;
    let t47981 = 0.21951497276451705329e-1_f64 * t47980;
    let t47985 = t10069 * t14231;
    let t48003 = t10139 * t5659 * t136 * t2457;
    let t48004 = 0.34697458558045176417e-2_f64 * t48003;
    let t48005 = t9303 * t14202;
    (t47971, t47979, t47981, t47985, t48004, t48005)
}
