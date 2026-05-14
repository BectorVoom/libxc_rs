//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 744/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk744<F: Float>(t2212: F, t2983: F, t12714: F, t11982: F, t3440: F, t3439: F, t157: F, t9224: F, t160: F, t7763: F, t11437: F, t1651: F, t3445: F, t2221: F, t1643: F, t9115: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12715 = t2983 * t2212;
    let t12716 = t12714 * t12715;
    let t12719 = t3440 * t11982;
    let t12720 = t3439 * t12719;
    let t12723 = t9224 * t157;
    let t12724 = t160 * t7763;
    let t12725 = t12724 * t11437;
    let t12726 = t12723 * t12725;
    let t12729 = t3445 * t1651;
    let t12730 = t2221 * t12729;
    let t12733 = t3445 * t1643;
    let t12734 = t9115 * t12733;
    (t12715, t12716, t12719, t12720, t12725, t12726, t12729, t12730, t12733, t12734)
}
