//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1090/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1090<F: Float>(t9931: F, t9934: F, t9936: F, t12740: F, t12741: F, t12742: F, t12743: F, t12745: F, t12746: F, t12748: F, t12749: F, t12750: F, t12751: F) -> (F, F, F, F) {
    let t12752 = F::new(8.0) / F::new(45.0) * t9931;
    let t12753 = F::new(16.0) / F::new(135.0) * t9934;
    let t12754 = F::new(8.0) / F::new(45.0) * t9936;
    let t12755 = t12740 - t12741 + t12742 - t12743 - t12745 - t12746 - t12748 + t12749 - t12750 - t12751 - t12752 - t12753 + t12754;
    (t12752, t12753, t12754, t12755)
}
