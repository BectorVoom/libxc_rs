//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 740/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk740(t2482: f64, t6985: f64, t2487: f64, t1445: f64, t6321: f64, t4529: f64, t874: f64, t1328: f64, t1555: f64, t894: f64, t2440: f64, t528: f64) -> (f64, f64, f64, f64, f64) {
    let t6986 = t6985 * t2482;
    let t6987 = t2487 * t6986;
    let t6989 = t1445 * t6321;
    let t6992 = t4529 * t874;
    let t6993 = t6992 * t1328;
    let t6994 = t1445 * t6993;
    let t6997 = t1555 * t894;
    let t7002 = t528 * t2440;
    (t6987, t6989, t6994, t6997, t7002)
}
