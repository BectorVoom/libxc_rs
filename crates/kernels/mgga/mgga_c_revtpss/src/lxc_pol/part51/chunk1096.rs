//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1096/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1096<F: Float>(t2014: F, t32297: F, t5542: F, t33657: F, t7235: F, t32114: F, t7898: F, t28021: F, t8568: F, t8567: F, t8995: F, t28199: F) -> (F, F, F, F, F) {
    let t125470 = t2014 * t32297 * t5542;
    let t125472 = F::new(3.0) * t7235 * t33657;
    let t125474 = F::new(2.0) * t7898 * t32114;
    let t125475 = t8568 * t28021;
    let t125478 = t8567 * t8995;
    let t125479 = t125478 * t28199;
    (t125470, t125472, t125474, t125475, t125479)
}
