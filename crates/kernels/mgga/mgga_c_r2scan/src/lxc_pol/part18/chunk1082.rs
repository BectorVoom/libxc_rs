//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1082/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1082<F: Float>(t38346: F, t3428: F, t3430: F, t6836: F, t10810: F, t870: F, t10684: F, t10648: F, t10958: F, t10971: F, t10962: F, t11477: F) -> (F, F, F, F, F, F, F) {
    let t38347 = F::new(0.91462949374725084942e-3) * t38346;
    let t38349 = t6836 * t3428 * t3430;
    let t38350 = F::new(0.15243824895787514157e-3) * t38349;
    let t38355 = t870 * t10810;
    let t38356 = t38355 * t10684;
    let t38359 = t10648 * t10971 * t10958;
    let t38362 = t10648 * t10971 * t10962;
    let t38363 = F::new(0.45731474687362542471e-3) * t38362;
    let t39149 = F::new(3.0) / F::new(2.0) * t11477;
    (t38347, t38350, t38355, t38356, t38359, t38363, t39149)
}
