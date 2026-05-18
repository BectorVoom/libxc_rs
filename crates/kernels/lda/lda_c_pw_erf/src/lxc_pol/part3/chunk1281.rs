//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1281/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1281<F: Float>(t12729: F, t12731: F, t12732: F, t12733: F, t12734: F, t12735: F, t12740: F, t12741: F, t12742: F, t12743: F, t12745: F, t12746: F, t12748: F) -> F {
    let t15044 = -t12729 + t12731 + t12732 - t12733 - t12734 + t12735 + t12740 - t12741 + t12742 - t12743 - t12745 - t12746 - t12748;
    t15044
}
