//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1261/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1261<F: Float>(t1124: F, t2363: F, t483: F, t485: F, t10766: F, t10768: F, t10772: F, t10775: F, t10778: F, t10783: F, t10787: F, t10788: F, t10791: F, t10793: F, t10800: F, t10802: F, t10805: F, t10808: F, t10812: F, t10816: F) -> (F,) {
    let t18755 = t1124 * t2363 * t483 * t485;
    let t18757 = -t10766 - 0.003950778065781896 * t10768 - 0.0004954275694490498 * t10772 - 0.01185233419734569 * t10775 - 0.002972565416694299 * t10778 - t10783 - t10787 - 0.051799090195807085 * t10788 - t10791 - t10793 + t10800 + t10802 + 0.006584630109636494 * t10805 + 0.03950778065781896 * t10808 + 0.006935985972286697 * t10812 + t10816 + 0.006584630109636494 * t18755;
    (t18757,)
}
