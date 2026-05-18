//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 661/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk661<F: Float>(t3243: F, t743: F, t1563: F, t34: F, t1697: F, t1832: F, t1852: F, t431: F, t156: F, t4: F, t411: F) -> (F, F, F, F, F) {
    let t5536 = t3243 * t743;
    let t5539 = t1563 * t34;
    let t5578 = t1697 * t1832;
    let t5592 = t431 * t1852;
    let t5594 = t4 * t156 * t411;
    (t5536, t5539, t5578, t5592, t5594)
}
