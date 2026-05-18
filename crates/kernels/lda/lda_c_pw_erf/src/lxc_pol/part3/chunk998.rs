//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 998/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk998<F: Float>(t1191: F, t163: F, t169: F, t841: F, t10766: F, t10768: F, t10772: F, t10775: F, t10778: F, t10783: F, t10787: F, t10788: F, t10791: F, t10793: F, t10796: F, t10800: F, t10802: F, t10805: F) -> F {
    let t11652 = t169 * t1191 * t841 * t163;
    let t11661 = F::new(0.0878110494085338) * t11652 - t10766 - F::new(0.01185233419734569) * t10768 - F::new(0.0014862827083471494) * t10772 - F::new(0.01777850129601853) * t10775 - F::new(0.004458848125041448) * t10778 - t10783 - t10787 - F::new(0.07769863529371063) * t10788 - t10791 - t10793 - F::new(0.001975389032890948) * t10796 + t10800 + t10802 + F::new(0.01975389032890948) * t10805;
    t11661
}
