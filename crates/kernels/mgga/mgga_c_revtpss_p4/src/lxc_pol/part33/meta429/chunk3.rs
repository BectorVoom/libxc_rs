//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1538/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1538<F: Float>(t19275: F, t953: F, t4669: F, t4673: F, t11452: F, t6157: F, t6190: F, t972: F, t11409: F, t11450: F, t15104: F, t15350: F, t15406: F, t15413: F, t19258: F, t19263: F, t19266: F, t19269: F, t19272: F, t2943: F, t2968: F, t3012: F, t4652: F, t4674: F, t4690: F, t4712: F) -> F {
    let t19276 = t19275 * t953;
    let t19279 = t4673 * t4669;
    let t19282 = t6157 * t11452;
    let t19283 = t19282 * t953;
    let t19290 = t6190 * t972;
    let t19293 = -t19258 - F::new(4.0) * t15104 * t4652 + F::cast_from(0.64327917994770140268e2_f64) * t15406 * t4674 + F::new(6.0) * t2968 * t19263 - F::new(4.0) * t2943 * t19266 - F::cast_from(0.19298375398431042081e3_f64) * t11409 * t19269 - F::new(2.0) * t2943 * t19272 + F::cast_from(0.32163958997385070134e2_f64) * t2968 * t19276 + F::cast_from(0.64327917994770140268e2_f64) * t2968 * t19279 + F::cast_from(0.2069040516770936012e4_f64) * t11450 * t19283 - F::cast_from(0.23392894490538584828e1_f64) * t15413 * t4690 + F::cast_from(0.34631718211362927517e2_f64) * t15350 * t4712 + F::cast_from(0.35089341735807877242e1_f64) * t3012 * t19290;
    t19293
}
