//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 960/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk960<F: Float>(t531: F, t7933: F, t7238: F, t2014: F, t1450: F, t5591: F, t7237: F, t13648: F, t2034: F, t25190: F, t7900: F, t5542: F, t7312: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28172 = t531 * t7933;
    let t28173 = t28172 * t7238;
    let t28175 = F::cast_from(3.0_f64) * t2014 * t28173;
    let t28176 = t1450 * t5591;
    let t28177 = t7237 * t28176;
    let t28179 = F::cast_from(3.0_f64) * t2014 * t28177;
    let t28182 = t2034 * t13648;
    let t28183 = t2014 * t28182;
    let t28184 = t25190 * t7900;
    let t28186 = F::cast_from(3.0_f64) * t2014 * t28184;
    let t28187 = t7312 * t5542;
    (t28173, t28175, t28176, t28177, t28179, t28182, t28183, t28184, t28186, t28187)
}
