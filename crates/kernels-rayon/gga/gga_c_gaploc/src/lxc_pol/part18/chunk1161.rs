//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1161/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1161(t1016: f64, t1382: f64, t7058: f64, t10624: f64, t1377: f64, t2761: f64, t6295: f64, t6525: f64, t10163: f64, t29874: f64, t2321: f64, t26673: f64, t9074: f64) -> (f64, f64, f64, f64, f64) {
    let t31483 = 2.0_f64 * t1382 * t1016 * t7058;
    let t31485 = 2.0_f64 * t1377 * t10624;
    let t31487 = t6525 * t2761 * t6295;
    let t31488 = 0.11856252764865062333e-2_f64 * t31487;
    let t31489 = t29874 * t10163;
    let t31490 = 0.23712505529730124666e-2_f64 * t31489;
    let t31492 = t9074 * t26673 * t2321;
    (t31483, t31485, t31488, t31490, t31492)
}
