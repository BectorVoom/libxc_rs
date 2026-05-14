//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1254/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1254<F: Float>(t18607: F, t18610: F, t18612: F, t18614: F, t18616: F, t18621: F, t18623: F, t18627: F, t18629: F, t18631: F, t18633: F, t18635: F, t18639: F, t18641: F, t18643: F, t18647: F) -> (F,) {
    let t18648 = -t18607 + t18610 + t18612 + t18614 - t18616 - t18621 - t18623 - t18627 - t18629 - t18631 + t18633 - t18635 - t18639 + t18641 + t18643 + t18647;
    (t18648,)
}
