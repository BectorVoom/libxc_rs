//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 670/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk670<F: Float>(t11227: F, t11229: F, t1869: F, t4811: F, t5205: F, t10513: F, t10515: F, t10517: F, t10525: F, t10527: F, t10530: F, t10532: F, t10537: F, t11190: F, t11197: F, t11201: F, t11204: F, t11209: F, t11211: F, t11216: F, t11222: F, t1693: F, t1792: F, t4830: F, t5044: F) -> (F, F, F) {
    let t11230 = t11227 * t11229;
    let t11231 = t1869 * t11230;
    let t11233 = t4811 * t5205;
    let t11235 = -0.74618749999999999998e-2 * t10513 + 0.33163888888888888887e-2 * t10515 - 0.16581944444444444444e-2 * t10517 + 0.16581944444444444444e-2 * t10525 + 0.66327777777777777776e-2 * t10527 - 0.16581944444444444444e-2 * t10530 - 0.49745833333333333332e-2 * t10532 - 0.49745833333333333332e-2 * t10537 - 0.193e0 * t1693 * t11190 - 0.579e0 * t4830 * t5044 - 0.43134342e-1 * t11197 * t11201 - 0.579e0 * t11204 * t1792 + 0.16581944444444444444e-2 * t11209 - 0.11054629629629629629e-2 * t11211 + 0.73697530864197530862e-3 * t11216 + 0.55273148148148148145e-2 * t11222 - 0.1492375e-1 * t11231 - 0.11054629629629629629e-2 * t11233;
    (t11231, t11233, t11235)
}
