//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1867/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1867(t19275: f64, t953: f64, t4669: f64, t4673: f64, t11452: f64, t6157: f64, t6190: f64, t972: f64, t11409: f64, t11450: f64, t15104: f64, t15350: f64, t15406: f64, t15413: f64, t19258: f64, t19263: f64, t19266: f64, t19269: f64, t19272: f64, t2943: f64, t2968: f64, t3012: f64, t4652: f64, t4674: f64, t4690: f64, t4712: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19276 = t19275 * t953;
    let t19279 = t4673 * t4669;
    let t19282 = t6157 * t11452;
    let t19283 = t19282 * t953;
    let t19290 = t6190 * t972;
    let t19293 = -t19258 - 4.0_f64 * t15104 * t4652 + 0.64327917994770140268e2_f64 * t15406 * t4674 + 6.0_f64 * t2968 * t19263 - 4.0_f64 * t2943 * t19266 - 0.19298375398431042081e3_f64 * t11409 * t19269 - 2.0_f64 * t2943 * t19272 + 0.32163958997385070134e2_f64 * t2968 * t19276 + 0.64327917994770140268e2_f64 * t2968 * t19279 + 0.2069040516770936012e4_f64 * t11450 * t19283 - 0.23392894490538584828e1_f64 * t15413 * t4690 + 0.34631718211362927517e2_f64 * t15350 * t4712 + 0.35089341735807877242e1_f64 * t3012 * t19290;
    (t19276, t19279, t19282, t19283, t19290, t19293)
}
