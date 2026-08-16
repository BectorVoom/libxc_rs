//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 362/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk362(t103: f64, t1320: f64, t566: f64, t1323: f64, t569: f64, t568: f64, t106: f64, t9: f64) -> (f64, f64, f64, f64) {
    let t1583 = t103 * t1320;
    let t1584 = t1583 * t566;
    let t1585 = t569 * t1323;
    let t1586 = t568 * t1585;
    let t1589 = t106 * t9;
    (t1583, t1584, t1586, t1589)
}
