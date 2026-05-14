//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 955/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk955<F: Float>(t32110: F, t7732: F, t121441: F, t2014: F, t7900: F, t33667: F, t7235: F, t32121: F, t7898: F, t25082: F, t27153: F, t36970: F, t13648: F, t8595: F, t33651: F, t7312: F) -> (F, F, F, F, F, F, F) {
    let t125507 = 2.0 * t7732 * t32110;
    let t125510 = 3.0 * t2014 * t121441 * t7900;
    let t125512 = 2.0 * t7235 * t33667;
    let t125514 = 3.0 * t7898 * t32121;
    let t125521 = 3.0 * t25082 * t36970 * t27153;
    let t125525 = t2014 * t8595 * t13648;
    let t125531 = 2.0 * t2014 * t7312 * t33651;
    (t125507, t125510, t125512, t125514, t125521, t125525, t125531)
}
