//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1223/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1223<F: Float>(t32356: F, t7290: F, t1841: F, t7289: F, t2554: F, t7064: F, t9006: F, t10667: F, t296: F, t10714: F, t7137: F, t10782: F, t1710: F) -> (F, F, F, F, F, F) {
    let t32357 = t7290 * t32356;
    let t32360 = F::cast_from(0.34180116578409885704e-2_f64) * t1841 * t7289 * t32357;
    let t32362 = t7064 * t9006 * t2554;
    let t32363 = F::cast_from(0.64087718584518535698e-3_f64) * t32362;
    let t32364 = t296 * t10667;
    let t32370 = F::cast_from(0.41016139894091862846e-1_f64) * t7137 * t10714;
    let t32371 = t10782 * t1710;
    (t32357, t32360, t32363, t32364, t32370, t32371)
}
