//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 586/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk586<F: Float>(t3484: F, t8176: F, t3482: F, t1341: F, t7740: F, t1340: F, t1339: F, t1220: F, t5610: F, t8064: F, t8075: F, t8080: F, t8084: F, t8087: F, t8091: F, t8095: F, t8165: F, t8173: F) -> (F, F, F, F, F, F) {
    let t8177 = t3484 * t8176;
    let t8178 = t3482 * t8177;
    let t8180 = t1341 * t7740;
    let t8181 = t1340 * t8180;
    let t8182 = t1339 * t8181;
    let t8184 = F::cast_from(0.49745833333333333332e-2_f64) * t8075 - F::cast_from(0.33163888888888888888e-2_f64) * t8080 - F::cast_from(0.55273148148148148147e-3_f64) * t8084 + F::cast_from(0.33163888888888888888e-2_f64) * t8087 + F::cast_from(0.16581944444444444444e-2_f64) * t8091 + F::cast_from(0.27636574074074074073e-2_f64) * t8095 + F::cast_from(0.24872916666666666666e-2_f64) * t8165 + F::cast_from(0.22109259259259259258e-2_f64) * t5610 + F::cast_from(0.193e0_f64) * t1220 * t8064 - F::cast_from(0.33163888888888888888e-2_f64) * t8173 + F::cast_from(0.22109259259259259258e-2_f64) * t8178 - F::cast_from(0.33163888888888888888e-2_f64) * t8182;
    (t8177, t8178, t8180, t8181, t8182, t8184)
}
