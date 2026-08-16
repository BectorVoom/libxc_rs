//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 723/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk723<F: Float>(t10513: F, t10515: F, t10517: F, t10525: F, t10527: F, t10530: F, t10532: F, t10537: F, t11190: F, t11197: F, t11201: F, t11204: F, t11209: F, t11211: F, t11216: F, t11222: F, t11231: F, t11233: F, t1693: F, t1792: F, t4830: F, t5044: F) -> F {
    let t11235 = -F::cast_from(0.74618749999999999998e-2_f64) * t10513 + F::cast_from(0.33163888888888888887e-2_f64) * t10515 - F::cast_from(0.16581944444444444444e-2_f64) * t10517 + F::cast_from(0.16581944444444444444e-2_f64) * t10525 + F::cast_from(0.66327777777777777776e-2_f64) * t10527 - F::cast_from(0.16581944444444444444e-2_f64) * t10530 - F::cast_from(0.49745833333333333332e-2_f64) * t10532 - F::cast_from(0.49745833333333333332e-2_f64) * t10537 - F::cast_from(0.193e0_f64) * t1693 * t11190 - F::cast_from(0.579e0_f64) * t4830 * t5044 - F::cast_from(0.43134342e-1_f64) * t11197 * t11201 - F::cast_from(0.579e0_f64) * t11204 * t1792 + F::cast_from(0.16581944444444444444e-2_f64) * t11209 - F::cast_from(0.11054629629629629629e-2_f64) * t11211 + F::cast_from(0.73697530864197530862e-3_f64) * t11216 + F::cast_from(0.55273148148148148145e-2_f64) * t11222 - F::cast_from(0.1492375e-1_f64) * t11231 - F::cast_from(0.11054629629629629629e-2_f64) * t11233;
    t11235
}
