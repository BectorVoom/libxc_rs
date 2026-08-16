//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1043/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1043(t15434: f64, t3138: f64, t979: f64, t2925: f64, t855: f64, t12630: f64, t136: f64, t15203: f64, t15217: f64, t15221: f64, t15226: f64, t15423: f64, t15428: f64, t15432: f64, t2927: f64, t2932: f64, t2936: f64, t3064: f64, t856: f64, t934: f64) -> (f64, f64) {
    let t15435 = t3138 * t15434;
    let t15436 = t979 * t15435;
    let t15445 = t2925 * t855;
    let t15450 = 0.223494e0_f64 * t15217 * t2936 + 0.223494e0_f64 * t2932 * t15221 - 0.10317654320987654321e0_f64 * t15226 - 0.193e0_f64 * t856 * t15423 - 0.99491666666666666664e-2_f64 * t15428 - 0.39796666666666666665e-1_f64 * t15432 + 0.59694999999999999999e-1_f64 * t15436 - 0.579e0_f64 * t2927 * t3064 + t12630 * t136 + 0.579e0_f64 * t856 * t15221 + 0.579e0_f64 * t2927 * t2936 - 0.579e0_f64 * t15445 * t934 - 0.386e0_f64 * t856 * t15203;
    (t15436, t15450)
}
