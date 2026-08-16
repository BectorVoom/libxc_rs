//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 359/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk359(t1340: f64, t531: f64, t1328: f64, t569: f64, t568: f64, t561: f64, t596: f64) -> (f64, f64, f64) {
    let t1573 = t531 * t1340;
    let t1576 = t569 * t1328;
    let t1577 = t568 * t1576;
    let t1580 = t561 * t596;
    (t1573, t1577, t1580)
}
