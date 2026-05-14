//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1360/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1360<F: Float>(t133: F, t19539: F, t19532: F, t14588: F, t1870: F, t19626: F, t19629: F, t19632: F, t19636: F, t19639: F, t19640: F, t19641: F, t19642: F, t19643: F, t19646: F, t19648: F, t19651: F, t19693: F) -> (F,) {
    let t19773 = t133 * t19539;
    let t19775 = t133 * t19532;
    let t19777 = 1.1495033333333333 * t14588 + t19626 + t19629 - t19632 - t19636 + t19639 - t19640 - t19641 - t19642 + t19643 + t19646 - t19648 - t19651 - 82.76424 * t1870 * t19693 - 0.7663355555555555 * t19773 + 2.2990066666666666 * t19775;
    (t19777,)
}
