//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2878/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2878<F: Float>(t41880: F, t4595: F, t15513: F, t914: F, t936: F, t15416: F, t2919: F, t2923: F, t4587: F, t2927: F, t11380: F, t4590: F) -> (F, F, F, F, F) {
    let t52213 = F::new(6.0) * t41880 * t4595;
    let t52214 = t15513 * t914;
    let t52216 = F::new(3.0) * t52214 * t936;
    let t52218 = F::new(3.0) * t15416 * t2919;
    let t52219 = t4587 * t2923;
    let t52221 = F::cast_from(0.48245938496077605201e2_f64) * t52219 * t2927;
    let t52223 = F::new(1.0) * t4590 * t11380;
    (t52213, t52216, t52218, t52221, t52223)
}
