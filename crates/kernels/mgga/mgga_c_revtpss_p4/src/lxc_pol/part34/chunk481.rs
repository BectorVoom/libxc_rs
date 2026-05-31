//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 481/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk481<F: Float>(t3143: F, t360: F, t368: F, t335: F, t365: F, t3141: F, t73: F) -> (F, F, F, F, F, F, F) {
    let t3144 = t3143 * t360;
    let t3145 = t368 * t368;
    let t3147 = F::cast_from(1.0_f64) / t3145 / t335;
    let t3148 = t365 * t3147;
    let t3149 = t3144 * t3148;
    let t3150 = t3141 * t3149;
    let t3153 = t73 * t73;
    (t3144, t3145, t3147, t3148, t3149, t3150, t3153)
}
