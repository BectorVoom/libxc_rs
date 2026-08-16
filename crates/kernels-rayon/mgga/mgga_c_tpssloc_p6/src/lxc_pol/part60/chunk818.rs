//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 818/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk818(t29274: f64, t29285: f64, t539: f64, t1807: f64, t7918: f64, t2085: f64, t6361: f64, t12021: f64, t2091: f64, t6439: f64, t1842: f64, t7936: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t29286 = t29274 + t29285;
    let t29287 = t539 * t29286;
    let t29290 = t1807 * t7918;
    let t29293 = t6361 * t2085;
    let t29299 = t12021 * t2091 * t6439;
    let t29310 = t7936 * t1842;
    (t29286, t29287, t29290, t29293, t29299, t29310)
}
