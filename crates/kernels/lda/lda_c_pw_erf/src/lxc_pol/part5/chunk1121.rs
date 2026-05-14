//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1121/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1121<F: Float>(t20768: F, t20773: F, t20775: F, t20776: F, t20781: F, t20784: F, t20787: F, t20791: F, t20795: F, t20800: F, t20804: F, t20807: F, t20812: F, t20816: F, t20819: F, t20826: F, t20829: F, t20832: F, t20835: F, t20837: F, t20840: F, t20844: F, t20848: F, t20850: F, t20852: F, t20854: F) -> (F, F) {
    let t23195 = t20768 - t20773 + t20775 + t20776 + t20781 - t20784 - t20787 - t20791 + t20795 - t20800 - t20804 + t20807 + t20812;
    let t23198 = t20816 - t20819 + t20826 - t20829 + t20832 - t20835 - t20837 - t20840 - t20844 + t20848 - t20850 + t20852 + t20854;
    (t23195, t23198)
}
