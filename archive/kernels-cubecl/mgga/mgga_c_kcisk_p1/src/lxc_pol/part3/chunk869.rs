//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 869/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk869<F: Float>(t1167: F, t3638: F, t3641: F, t1191: F, t12911: F, t3677: F, t1192: F, t3671: F, t3639: F, t1190: F, t3679: F, t330: F, t3721: F) -> (F, F, F, F, F) {
    let t13048 = t1167 * t3638;
    let t13050 = F::cast_from(6.0_f64) * t13048 * t3641;
    let t13051 = t12911 * t1191;
    let t13053 = F::cast_from(6.0_f64) * t3677 * t13051;
    let t13054 = t1192 * t3671;
    let t13056 = F::cast_from(6.0_f64) * t3639 * t13054;
    let t13058 = t3671 * t3679 * t1190;
    let t13060 = F::cast_from(0.48245472966453314466e2_f64) * t3677 * t13058;
    let t13064 = F::cast_from(1.0_f64) / t3721 / t330;
    (t13050, t13053, t13056, t13060, t13064)
}
