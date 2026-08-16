//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1070/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1070(t5558: f64, t952: f64, t1959: f64, t2590: f64, t119: f64, t19077: f64, t481: f64, t19223: f64, t19244: f64, t1570: f64, t21488: f64, t565: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t23555 = t952 * t5558;
    let t23575 = t2590 * t1959;
    let t23609 = t481 * t19077 * t119;
    let t23726 = t481 * t19223 * t119;
    let t23741 = t481 * t19244 * t119;
    let t23759 = t21488 * t565 * t1570;
    (t23555, t23575, t23609, t23726, t23741, t23759)
}
