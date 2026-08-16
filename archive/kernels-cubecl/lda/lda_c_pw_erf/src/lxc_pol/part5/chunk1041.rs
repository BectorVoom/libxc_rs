//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1041/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1041<F: Float>(t2565: F, t3783: F, t519: F, t2539: F, t3762: F, t571: F, t10313: F, t2553: F, t518: F, t6610: F, t4763: F, t5363: F) -> (F, F, F, F, F) {
    let t18593 = t519 * t3783 * t2565;
    let t18596 = t571 * t3762 * t2539;
    let t18599 = t519 * t10313 * t2553;
    let t18608 = t6610 * t518;
    let t18615 = t4763 * t5363;
    (t18593, t18596, t18599, t18608, t18615)
}
