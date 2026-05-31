//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1187/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk1187<F: Float>(t38: F, t5980: F, t776: F, t342: F, t7317: F, t11230: F, t21410: F, t21423: F, t21439: F, t21442: F, t21445: F, t2209: F, t2229: F, t2448: F, t5740: F, t63: F, t6989: F, t7277: F, t8245: F) -> (F, F, F) {
    let t21448 = F::cast_from(17.53815_f64) * t38 * t776 * t5980;
    let t21451 = F::cast_from(5.84605_f64) * t38 * t7317 * t342;
    let t21452 = -F::cast_from(88.1424_f64) * t11230 * t21410 - t21423 + F::cast_from(176.2848_f64) * t63 * t8245 * t7277 * t342 - F::cast_from(88.1424_f64) * t63 * t6989 * t2209 + F::cast_from(17.62848_f64) * t63 * t5740 * t2448 + F::cast_from(17.62848_f64) * t63 * t2229 * t5980 + t21439 - t21442 + t21445 + t21448 + t21451;
    (t21448, t21451, t21452)
}
