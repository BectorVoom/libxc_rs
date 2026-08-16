//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1026/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1026(t15116: f64, t15132: f64, t15149: f64, t15165: f64, t14283: f64, t14286: f64, t14289: f64, t14291: f64, t14297: f64, t14300: f64, t14601: f64, t15082: f64, t15084: f64, t15087: f64, t15094: f64, t15095: f64, t15098: f64, t1611: f64, t1620: f64, t4530: f64, t4535: f64, t4536: f64, t4565: f64, t555: f64) -> f64 {
    let t15167 = t15116 + t15132 + t15149 + t15165;
    let t15169 = t15082 * t555 - 3.0_f64 * t15084 * t1620 + 6.0_f64 * t15087 * t4536 - 6.0_f64 * t15094 * t15095 + 6.0_f64 * t15098 * t4535 - t15167 * t1611 - 3.0_f64 * t4530 * t4565 - t14283 + t14286 - t14289 + t14291 + t14297 - t14300 + t14601;
    t15169
}
