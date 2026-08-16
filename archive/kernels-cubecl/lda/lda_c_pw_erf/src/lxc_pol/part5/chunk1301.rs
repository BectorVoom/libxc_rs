//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1301/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1301<F: Float>(t20768: F, t20773: F, t20775: F, t20776: F, t20781: F, t20784: F, t20787: F, t20791: F, t20795: F, t20800: F, t20804: F, t20807: F, t20812: F) -> F {
    let t23195 = t20768 - t20773 + t20775 + t20776 + t20781 - t20784 - t20787 - t20791 + t20795 - t20800 - t20804 + t20807 + t20812;
    t23195
}
