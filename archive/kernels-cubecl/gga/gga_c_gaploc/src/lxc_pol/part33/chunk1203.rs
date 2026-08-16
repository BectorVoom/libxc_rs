//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1203/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1203<F: Float>(t32356: F, t7290: F, t1841: F, t7289: F, t2554: F, t7064: F, t9006: F, t10714: F, t7137: F, t21636: F, t3440: F, t3420: F) -> (F, F, F, F, F, F) {
    let t32357 = t7290 * t32356;
    let t32360 = F::cast_from(0.34180116578409885704e-2_f64) * t1841 * t7289 * t32357;
    let t32362 = t7064 * t9006 * t2554;
    let t32363 = F::cast_from(0.64087718584518535698e-3_f64) * t32362;
    let t32370 = F::cast_from(0.41016139894091862846e-1_f64) * t7137 * t10714;
    let t32394 = F::cast_from(0.10254034973522965712e-1_f64) * t21636 * t3440;
    let t32398 = F::cast_from(0.34180116578409885707e-2_f64) * t21636 * t3420;
    (t32357, t32360, t32363, t32370, t32394, t32398)
}
