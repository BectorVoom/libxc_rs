//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2034/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2034(t225: f64, t29287: f64, t16439: f64, t19647: f64, t19648: f64, t20029: f64, t2092: f64, t24095: f64, t26224: f64, t26989: f64, t29361: f64, t3758: f64, t5210: f64, t56607: f64, t568: f64, t6461: f64, t7194: f64, t7199: f64, t7918: f64, t7937: f64, t84705: f64, t91548: f64, t97766: f64) -> (f64, f64) {
    let t102948 = t29287 * t225;
    let t102972 = -t3758 * t29361 + 0.6579736267392905746e-1_f64 * t91548 + 2.0_f64 * t5210 * t7918 * t568 - t24095 * t6461 - 0.6579736267392905746e-1_f64 * t97766 + 4.0_f64 * t7194 * t19648 - t84705 - 2.0_f64 * t56607 * t2092 - 12.0_f64 * t26224 * t26989 * t19647 + 4.0_f64 * t20029 * t7199 - 2.0_f64 * t16439 * t7937;
    (t102948, t102972)
}
