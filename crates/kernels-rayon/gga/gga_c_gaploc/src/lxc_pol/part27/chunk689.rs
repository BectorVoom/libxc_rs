//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 689/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk689(t2343: f64, t6443: f64, t2293: f64, t555: f64, t494: f64, t2312: f64, t2327: f64, t4245: f64, t883: f64, t485: f64, t1320: f64, t481: f64, t880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6444 = t2343 * t6443;
    let t6447 = t555 * t2293;
    let t6448 = t6447 * t494;
    let t6451 = t2312 * t2327;
    let t6455 = t883 * t4245;
    let t6456 = t485 * t6455;
    let t6457 = t481 * t880 * t1320 * t6456;
    (t6444, t6447, t6448, t6451, t6455, t6457)
}
