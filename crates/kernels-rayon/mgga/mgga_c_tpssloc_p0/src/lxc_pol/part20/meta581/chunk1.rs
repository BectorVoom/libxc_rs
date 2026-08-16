//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2149/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2149(t10908: f64, t3114: f64, t1036: f64, t10438: f64, t221: f64, t339: f64, t42813: f64, t10283: f64, t995: f64, t10931: f64, t135: f64, t973: f64) -> (f64, f64, f64, f64, f64) {
    let t43301 = t3114 * t10908;
    let t43303 = t10438 * t1036;
    let t43307 = 5.0_f64 / 486.0_f64 * t339 * t221 * t42813;
    let t43310 = t10283 * t995;
    let t43313 = t973 * t135 * t10931;
    (t43301, t43303, t43307, t43310, t43313)
}
