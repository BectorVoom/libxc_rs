//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2045/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2045<F: Float>(t102070: F, t109096: F, t110110: F, t110853: F, t111004: F, t111039: F, t111068: F, t118: F, t13648: F, t2014: F, t2089: F, t21814: F, t21891: F, t22287: F, t22496: F, t2322: F, t25082: F, t26399: F, t26405: F, t26411: F, t27833: F, t28167: F, t28196: F, t28658: F, t28711: F, t28932: F, t29494: F, t30209: F, t30315: F, t34495: F, t569: F, t5877: F, t5887: F, t671: F, t7235: F, t7359: F, t7474: F, t7732: F, t7898: F, t8108: F, t8111: F, t86771: F, t9069: F) -> F {
    let t111089 = F::new(6.0) * t28167 * t9069 * t22287 + t7235 * t30315 - t21814 * t2089 - t5877 * t7474 - F::new(2.0) * t110110 * t671 - F::new(6.0) * t28196 * t102070 * t109096 + F::new(3.0) * t2014 * t26411 * t29494 - t118 * (t110853 + t111004) + F::new(6.0) * t7898 * t28932 - F::new(6.0) * t25082 * t34495 * t22496 + (t111039 + t111068) * t569 - F::new(2.0) * t2014 * t8108 * t13648 - F::new(2.0) * t27833 * t8111 - F::new(3.0) * t25082 * t26405 * t86771 - F::new(4.0) * t26399 * t5887 - F::new(4.0) * t28658 * t5887 - F::new(4.0) * t7359 * t21891 - F::new(4.0) * t2322 * t30209 - F::new(4.0) * t7732 * t28711;
    t111089
}
