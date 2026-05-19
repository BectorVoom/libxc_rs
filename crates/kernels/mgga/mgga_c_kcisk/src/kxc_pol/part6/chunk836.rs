//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 836/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk836<F: Float>(t28229: F, t5192: F, t5182: F, t2364: F, t8946: F, t10426: F, t28208: F, t6675: F, t5184: F, t6674: F, t15951: F, t15953: F, t15955: F, t22265: F, t22328: F, t28211: F, t28219: F, t28222: F, t28226: F) -> (F, F, F, F) {
    let t28230 = t5192 * t28229;
    let t28231 = t5182 * t28230;
    let t28233 = t8946 * t2364;
    let t28234 = t10426 * t28233;
    let t28235 = t5182 * t28234;
    let t28237 = t6675 * t28208;
    let t28238 = t5184 * t28237;
    let t28239 = t6674 * t28238;
    let t28241 = F::cast_from(0.48640370370370370369e-1_f64) * t22265 + F::cast_from(0.99491666666666666664e-2_f64) * t28211 + F::cast_from(0.16581944444444444444e-2_f64) * t15951 - F::cast_from(0.11054629629629629629e-2_f64) * t15953 + F::cast_from(0.44218518518518518518e-2_f64) * t15955 + F::cast_from(0.44218518518518518516e-2_f64) * t22328 - F::cast_from(0.11054629629629629629e-2_f64) * t28219 - F::cast_from(0.17687407407407407407e-1_f64) * t28222 + F::cast_from(0.33163888888888888887e-2_f64) * t28226 - F::cast_from(0.66327777777777777775e-2_f64) * t28231 - F::cast_from(0.66327777777777777776e-2_f64) * t28235 - F::cast_from(0.8290972222222222222e-2_f64) * t28239;
    (t28231, t28235, t28239, t28241)
}
