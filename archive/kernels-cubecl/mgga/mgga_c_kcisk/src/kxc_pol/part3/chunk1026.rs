//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1026/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1026<F: Float>(t15116: F, t15132: F, t15149: F, t15165: F, t14283: F, t14286: F, t14289: F, t14291: F, t14297: F, t14300: F, t14601: F, t15082: F, t15084: F, t15087: F, t15094: F, t15095: F, t15098: F, t1611: F, t1620: F, t4530: F, t4535: F, t4536: F, t4565: F, t555: F) -> F {
    let t15167 = t15116 + t15132 + t15149 + t15165;
    let t15169 = t15082 * t555 - F::cast_from(3.0_f64) * t15084 * t1620 + F::cast_from(6.0_f64) * t15087 * t4536 - F::cast_from(6.0_f64) * t15094 * t15095 + F::cast_from(6.0_f64) * t15098 * t4535 - t15167 * t1611 - F::cast_from(3.0_f64) * t4530 * t4565 - t14283 + t14286 - t14289 + t14291 + t14297 - t14300 + t14601;
    t15169
}
