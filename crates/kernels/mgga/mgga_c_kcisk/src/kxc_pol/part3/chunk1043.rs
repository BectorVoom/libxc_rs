//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1043/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1043<F: Float>(t15434: F, t3138: F, t979: F, t2925: F, t855: F, t12630: F, t136: F, t15203: F, t15217: F, t15221: F, t15226: F, t15423: F, t15428: F, t15432: F, t2927: F, t2932: F, t2936: F, t3064: F, t856: F, t934: F) -> (F, F) {
    let t15435 = t3138 * t15434;
    let t15436 = t979 * t15435;
    let t15445 = t2925 * t855;
    let t15450 = F::cast_from(0.223494e0_f64) * t15217 * t2936 + F::cast_from(0.223494e0_f64) * t2932 * t15221 - F::cast_from(0.10317654320987654321e0_f64) * t15226 - F::cast_from(0.193e0_f64) * t856 * t15423 - F::cast_from(0.99491666666666666664e-2_f64) * t15428 - F::cast_from(0.39796666666666666665e-1_f64) * t15432 + F::cast_from(0.59694999999999999999e-1_f64) * t15436 - F::cast_from(0.579e0_f64) * t2927 * t3064 + t12630 * t136 + F::cast_from(0.579e0_f64) * t856 * t15221 + F::cast_from(0.579e0_f64) * t2927 * t2936 - F::cast_from(0.579e0_f64) * t15445 * t934 - F::cast_from(0.386e0_f64) * t856 * t15203;
    (t15436, t15450)
}
