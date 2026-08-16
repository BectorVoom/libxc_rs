//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1300/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1300<F: Float>(t20725: F, t20732: F, t20736: F, t20740: F, t20743: F, t20747: F, t20751: F, t20753: F, t20755: F, t20757: F, t20759: F, t20762: F, t20765: F) -> F {
    let t23194 = -t20725 + t20732 - t20736 + t20740 - t20743 - t20747 + t20751 - t20753 + t20755 + t20757 - t20759 - t20762 + t20765;
    t23194
}
