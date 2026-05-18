//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 866/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk866<F: Float>(t28176: F, t7237: F, t2014: F, t13648: F, t2034: F, t25190: F, t7900: F, t5542: F, t7312: F, t7315: F, t7934: F, t7235: F, t7901: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t28177 = t7237 * t28176;
    let t28179 = F::new(3.0) * t2014 * t28177;
    let t28182 = t2034 * t13648;
    let t28183 = t2014 * t28182;
    let t28184 = t25190 * t7900;
    let t28186 = F::new(3.0) * t2014 * t28184;
    let t28187 = t7312 * t5542;
    let t28188 = t2014 * t28187;
    let t28189 = t7934 * t7315;
    let t28190 = t2014 * t28189;
    let t28192 = F::new(3.0) * t7235 * t7901;
    (t28177, t28179, t28182, t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192)
}
