//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1160/1363 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1160<F: Float>(t11409: F, t11450: F, t15104: F, t15350: F, t15406: F, t15413: F, t19258: F, t19263: F, t19266: F, t19269: F, t19272: F, t19276: F, t19279: F, t19283: F, t19290: F, t2943: F, t2968: F, t3012: F, t4652: F, t4674: F, t4690: F, t4712: F) -> (F,) {
    let t19293 = -t19258 - 4.0 * t15104 * t4652 + 0.64327917994770140268e2 * t15406 * t4674 + 6.0 * t2968 * t19263 - 4.0 * t2943 * t19266 - 0.19298375398431042081e3 * t11409 * t19269 - 2.0 * t2943 * t19272 + 0.32163958997385070134e2 * t2968 * t19276 + 0.64327917994770140268e2 * t2968 * t19279 + 0.2069040516770936012e4 * t11450 * t19283 - 0.23392894490538584828e1 * t15413 * t4690 + 0.34631718211362927517e2 * t15350 * t4712 + 0.35089341735807877242e1 * t3012 * t19290;
    (t19293,)
}
