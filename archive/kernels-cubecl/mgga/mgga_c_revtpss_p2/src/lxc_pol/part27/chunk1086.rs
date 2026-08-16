//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1086/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1086<F: Float>(t196: F, t197: F, t3821: F, t2035: F, t531: F, t7311: F, t7238: F, t2014: F, t7312: F, t7315: F, t2394: F, t30: F) -> (F, F, F, F, F, F, F, F) {
    let t25188 = t3821 * t196 * t197;
    let t25189 = t25188 * t2035;
    let t25190 = t531 * t7311;
    let t25191 = t25190 * t7238;
    let t25193 = F::cast_from(6.0_f64) * t2014 * t25191;
    let t25194 = t7312 * t7315;
    let t25196 = F::cast_from(2.0_f64) * t2014 * t25194;
    let t25198 = t30 * t2394;
    (t25188, t25189, t25190, t25191, t25193, t25194, t25196, t25198)
}
