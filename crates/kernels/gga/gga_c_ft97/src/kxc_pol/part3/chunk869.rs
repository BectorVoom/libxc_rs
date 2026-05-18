//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 869/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk869<F: Float>(t16976: F, t17038: F, t17098: F, t17144: F, t17194: F, t17373: F, t17425: F, t17504: F, t1023: F, t1058: F, t149: F, t165: F, t16659: F, t16661: F, t16664: F, t16932: F, t3313: F, t3414: F, t3588: F, t4650: F, t4720: F, t4837: F, t564: F, t614: F) -> F {
    let t17507 = t16976 + t17038 + t17098 + t17144 + t17194 + t17373 + t17425 + t17504;
    let t17509 = -F::new(2.0) * t1023 * t3588 - F::new(2.0) * t1058 * t3313 - F::new(2.0) * t1058 * t3414 - t149 * t17507 - t165 * t16659 - t165 * t16661 - t165 * t16664 - t165 * t16932 - t4650 * t614 - t4720 * t614 - t4837 * t564;
    t17509
}
