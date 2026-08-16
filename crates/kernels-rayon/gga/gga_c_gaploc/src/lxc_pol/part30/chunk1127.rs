//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1127/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1127(t6466: f64, t9074: f64, t9204: f64, t20572: f64, t2325: f64, t882: f64, t883: f64, t2312: f64, t9090: f64, t2321: f64, t6776: f64, t3122: f64, t6338: f64) -> (f64, f64, f64, f64, f64) {
    let t30014 = 0.71137516589190373998e-2_f64 * t9074 * t9204 * t6466;
    let t30049 = 0.23712505529730124666e-2_f64 * t882 * t2325 * t883 * t20572;
    let t30091 = 0.47425011059460249332e-2_f64 * t2312 * t9090;
    let t30094 = 0.23712505529730124666e-2_f64 * t882 * t6776 * t2321;
    let t30096 = 0.23712505529730124666e-2_f64 * t6338 * t3122;
    (t30014, t30049, t30091, t30094, t30096)
}
