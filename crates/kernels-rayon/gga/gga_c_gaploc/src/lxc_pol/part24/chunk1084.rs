//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1084/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1084(t107: f64, t25760: f64, t544: f64, t4360: f64, t8410: f64, t1359: f64, t2754: f64, t4149: f64, t986: f64, t1397: f64, t8330: f64, t1415: f64, t8265: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26455 = t544 * t25760 * t107;
    let t26609 = t4360 * t8410;
    let t26629 = t1359 * t2754;
    let t26673 = t4149 * t986;
    let t26726 = t1397 * t8330;
    let t26763 = t1415 * t8265;
    (t26455, t26609, t26629, t26673, t26726, t26763)
}
