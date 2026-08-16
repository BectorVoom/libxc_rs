//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1214/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1214(t1041: f64, t248: f64, t43338: f64, t5677: f64, t3070: f64, t43198: f64, t5908: f64, t5884: f64, t698: f64, t973: f64, t5889: f64, t5893: f64) -> (f64, f64, f64, f64, f64) {
    let t62445 = t1041 * t248 * t43338 * t5677;
    let t62494 = t3070 * t43198 * t5908;
    let t62559 = t973 * t698 * t5884;
    let t62565 = t973 * t698 * t5889;
    let t62832 = t973 * t698 * t5893;
    (t62445, t62494, t62559, t62565, t62832)
}
