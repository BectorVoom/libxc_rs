//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 705/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk705<F: Float>(t39: F, t780: F, t159: F, t285: F, t1549: F, t1809: F, t1729: F, t776: F, t2306: F, t684: F, t2310: F, t1738: F, t872: F) -> (F, F, F, F, F, F, F) {
    let t4437 = t39 * t780;
    let t4439 = t4437 * t159 * t285;
    let t4441 = t1549 * t1809;
    let t4449 = t1729 * t776;
    let t4454 = F::new(0.039914113367515366) * t684 * t2306;
    let t4455 = t684 * t2310;
    let t4457 = t1738 * t872;
    (t4437, t4439, t4441, t4449, t4454, t4455, t4457)
}
