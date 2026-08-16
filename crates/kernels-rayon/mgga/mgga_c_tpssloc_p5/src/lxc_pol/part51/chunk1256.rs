//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1256/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1256(t1373: f64, t254: f64, t225: f64, t26219: f64, t214: f64, t5318: f64, t1824: f64, t6955: f64, t2006: f64, t5286: f64, t26221: f64, t26329: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90665 = t1373 * t254;
    let t90732 = t26219 * t225;
    let t90739 = t214 * t5318;
    let t90942 = t6955 * t1824;
    let t90946 = t2006 * t5286;
    let t91441 = t26221 * t225;
    let t91488 = t26329 * t225;
    (t90665, t90732, t90739, t90942, t90946, t91441, t91488)
}
