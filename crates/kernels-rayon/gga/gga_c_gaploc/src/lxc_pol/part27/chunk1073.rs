//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1073/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1073(t119: f64, t19223: f64, t481: f64, t19244: f64, t1570: f64, t21488: f64, t565: f64, t555: f64, t189: f64, t20369: f64, t6508: f64, t2310: f64, t424: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23726 = t481 * t19223 * t119;
    let t23741 = t481 * t19244 * t119;
    let t23759 = t21488 * t565 * t1570;
    let t23763 = t21488 * t565 * t555;
    let t23767 = t21488 * t565 * t189;
    let t23911 = t6508 * t20369;
    let t23927 = t481 * t2310 * t424;
    (t23726, t23741, t23759, t23763, t23767, t23911, t23927)
}
