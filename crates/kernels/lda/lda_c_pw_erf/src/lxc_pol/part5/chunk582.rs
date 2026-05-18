//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 582/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk582<F: Float>(t265: F, t3704: F, t219: F, t3604: F, t197: F, t3476: F, t155: F, t573: F) -> (F, F, F, F) {
    let t3706 = F::new(8.0) / F::new(405.0) * t265 * t3704;
    let t3714 = t219 * t3604;
    let t3722 = t197 * t3476;
    let t3762 = t155 * t573;
    (t3706, t3714, t3722, t3762)
}
