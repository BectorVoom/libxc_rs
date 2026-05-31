//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1137/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1137<F: Float>(t32121: F, t7898: F, t32103: F, t7732: F, t2322: F, t33591: F, t25082: F, t27153: F, t36970: F, t32322: F, t7937: F, t13648: F, t2014: F, t8595: F) -> (F, F, F, F, F, F) {
    let t125514 = F::cast_from(3.0_f64) * t7898 * t32121;
    let t125515 = t7732 * t32103;
    let t125517 = t2322 * t33591;
    let t125521 = F::cast_from(3.0_f64) * t25082 * t36970 * t27153;
    let t125522 = t32322 * t7937;
    let t125525 = t2014 * t8595 * t13648;
    (t125514, t125515, t125517, t125521, t125522, t125525)
}
