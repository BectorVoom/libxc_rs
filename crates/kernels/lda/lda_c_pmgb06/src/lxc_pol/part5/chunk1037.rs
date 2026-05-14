//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1037/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1037<F: Float>(t8306: F, t8355: F, t11234: F, t21410: F, t342: F, t38: F, t7321: F, t2209: F, t2703: F, t2221: F, t2448: F, t5980: F, t776: F, t7317: F, t11230: F, t2229: F, t5740: F, t63: F, t6989: F, t7277: F, t8245: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21414 = 1.2991222222222223 * t8306;
    let t21416 = 1.5156425925925925 * t8355;
    let t21423 = 52.61445 * t11234 * t21410;
    let t21439 = 70.1526 * t38 * t7321 * t342;
    let t21442 = 52.61445 * t38 * t2703 * t2209;
    let t21445 = 17.53815 * t38 * t2221 * t2448;
    let t21448 = 17.53815 * t38 * t776 * t5980;
    let t21451 = 5.84605 * t38 * t7317 * t342;
    let t21452 = -88.1424 * t11230 * t21410 - t21423 + 176.2848 * t63 * t8245 * t7277 * t342 - 88.1424 * t63 * t6989 * t2209 + 17.62848 * t63 * t5740 * t2448 + 17.62848 * t63 * t2229 * t5980 + t21439 - t21442 + t21445 + t21448 + t21451;
    (t21414, t21416, t21423, t21439, t21442, t21445, t21448, t21451, t21452)
}
