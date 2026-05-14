//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 1077/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk1077<F: Float>(t14667: F, t133: F, t14582: F, t14585: F, t14588: F, t14634: F, t14641: F, t14644: F, t14648: F, t14652: F, t14656: F, t14660: F, t14661: F, t1664: F, t1832: F, t1870: F, t5651: F) -> (F, F) {
    let t14668 = 8.769075 * t14667;
    let t14673 = 1.7881162962962962 * t14582 - 2.2990066666666666 * t14585 + 1.724255 * t14588 - 1.724255 * t133 * t14634 - t14641 - t14644 - t14648 + t14652 + t14656 - t14660 - 62.07318 * t1870 * t14661 + t14668 - 62.07318 * t1870 * t5651 * t1832 * t1664;
    (t14668, t14673)
}
