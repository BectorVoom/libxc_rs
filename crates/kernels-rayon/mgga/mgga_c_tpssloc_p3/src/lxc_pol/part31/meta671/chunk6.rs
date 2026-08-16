//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2007/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2007(t5: f64, t102145: f64, t102171: f64, t102198: f64, t102223: f64, t102252: f64, t102278: f64, t102284: f64, t102305: f64, t112: f64, t19450: f64, t19577: f64, t19596: f64, t1983: f64, t19994: f64, t20098: f64, t20109: f64, t2040: f64, t2075: f64, t2079: f64, t22574: f64, t23938: f64, t24432: f64, t24987: f64, t24995: f64, t26898: f64, t26977: f64, t27144: f64, t27145: f64, t28821: f64, t29222: f64, t33899: f64, t510: f64, t5161: f64, t5460: f64, t6876: f64, t7042: f64, t7170: f64, t7171: f64, t7217: f64, t74032: f64, t75203: f64, t75560: f64, t7685: f64, t7904: f64, t9016: f64, t96824: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t102309 = piecewise3(t8, 0.0_f64, t102145 + t102171 + t102198 + t102223 + t102252 + t102278 + t102284 + t102305);
    let t102310 = t102309 * t112;
    let t102320 = 3.0_f64 * t28821 * t7171 + 6.0_f64 * t24995 * t9016 * t19994 - t19450 * t2075 + 3.0_f64 * t1983 * t7170 * t96824 + 6.0_f64 * t7685 * t26898 - t1983 * t7217 * t19596 - 6.0_f64 * t22574 * t33899 * t19577 - 2.0_f64 * t1983 * t27144 * t5161 + 2.0_f64 * t7685 * t27145 - t6876 * t29222 - 4.0_f64 * t23938 * t5460 - 4.0_f64 * t26977 * t5460 - 4.0_f64 * t7042 * t20109 - 6.0_f64 * t24995 * t24432 * t75203 - t102310 * t510 + t2079 * t20098 + 6.0_f64 * t24987 * t7904 - 3.0_f64 * t22574 * t24432 * t74032 - 2.0_f64 * t75560 * t2040;
    (t102310, t102320)
}
