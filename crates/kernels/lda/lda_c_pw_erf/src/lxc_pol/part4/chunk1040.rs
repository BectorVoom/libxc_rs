//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1040/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1040<F: Float>(t10832: F, t1872: F, t5673: F, t684: F, t2765: F, t5647: F, t5643: F, t159: F, t1904: F, t285: F, t39: F, t443: F, t5616: F, t1125: F, t763: F, t133: F) -> (F, F, F, F, F, F, F, F) {
    let t14500 = t10832 * t1872;
    let t14503 = t684 * t5673;
    let t14505 = t2765 * t5647;
    let t14508 = t2765 * t5643;
    let t14515 = t39 * t1904 * t159 * t285;
    let t14535 = t5616 * t443;
    let t14581 = t1125 * t763;
    let t14582 = t133 * t14581;
    (t14500, t14503, t14505, t14508, t14515, t14535, t14581, t14582)
}
