//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 655/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk655<F: Float>(t39: F, t780: F, t159: F, t285: F, t1549: F, t1809: F, t1729: F, t776: F, t2306: F, t684: F, t2310: F, t1738: F, t872: F, t1733: F, t2211: F, t2764: F, t2772: F, t2779: F, t2799: F, t2801: F, t2811: F, t4425: F, t4427: F, t4430: F, t4435: F, t777: F) -> (F, F, F, F) {
    let t4437 = t39 * t780;
    let t4439 = t4437 * t159 * t285;
    let t4441 = t1549 * t1809;
    let t4449 = t1729 * t776;
    let t4454 = 0.039914113367515366 * t684 * t2306;
    let t4455 = t684 * t2310;
    let t4457 = t1738 * t872;
    let t4459 = -t4425 - 0.0005811348303577384 * t4427 - 3.0 * t2764 * t4430 + 0.19816831758676853 * t4435 + 0.001355981270834723 * t4439 + 3.0 * t1733 * t4441 - t777 * t2799 + 2.0 * t777 * t2779 + 3.0 * t2211 * t2801 + 6.0 * t4449 * t2811 - 0.054045904796391424 * t2772 + t4454 + 0.039914113367515366 * t4455 - 0.05321881782335382 * t4457;
    (t4437, t4441, t4449, t4459)
}
