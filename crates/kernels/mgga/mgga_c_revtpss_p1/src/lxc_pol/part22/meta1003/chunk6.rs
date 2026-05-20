//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3424/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3424<F: Float>(t52163: F, t52482: F, t934: F, t15390: F, t52514: F, t19056: F, t2919: F, t2923: F, t6104: F, t2927: F, t1610: F, t52214: F) -> (F, F, F, F, F) {
    let t64327 = F::cast_from(0.2069040516770936012e4_f64) * t52482 * t52163 * t934;
    let t64329 = F::cast_from(0.38596750796862084161e3_f64) * t52514 * t15390;
    let t64335 = F::new(1.0) * t19056 * t2919;
    let t64336 = t6104 * t2923;
    let t64338 = F::cast_from(0.16081979498692535067e2_f64) * t64336 * t2927;
    let t64340 = F::new(2.0) * t52214 * t1610;
    (t64327, t64329, t64335, t64338, t64340)
}
