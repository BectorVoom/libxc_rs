//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1881/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1881<F: Float>(t2014: F, t28182: F, t25190: F, t7900: F, t5542: F, t7312: F, t7315: F, t7934: F, t7235: F, t7901: F, t7937: F, t2013: F, t8995: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28183 = t2014 * t28182;
    let t28184 = t25190 * t7900;
    let t28186 = F::cast_from(3.0_f64) * t2014 * t28184;
    let t28187 = t7312 * t5542;
    let t28188 = t2014 * t28187;
    let t28189 = t7934 * t7315;
    let t28190 = t2014 * t28189;
    let t28192 = F::cast_from(3.0_f64) * t7235 * t7901;
    let t28193 = t7235 * t7937;
    let t28196 = t2013 * t8995;
    (t28183, t28184, t28186, t28187, t28188, t28189, t28190, t28192, t28193, t28196)
}
