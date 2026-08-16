//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 942/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk942<F: Float>(t29653: F, t29667: F, t1974: F, t12061: F, t29636: F, t12002: F, t15989: F, t22564: F, t22575: F, t22583: F, t28371: F, t28375: F, t28379: F, t28383: F, t28387: F, t28391: F) -> (F, F, F) {
    let t29668 = t29653 + t29667;
    let t29669 = t29668 * t1974;
    let t29672 = t29636 * t12061;
    let t29685 = -t12002 - F::cast_from(0.2283111111111111111e-1_f64) * t15989 + F::cast_from(0.11415555555555555555e-1_f64) * t22564 - F::cast_from(0.34246666666666666665e-1_f64) * t22575 + F::cast_from(0.17123333333333333333e-1_f64) * t22583 - F::cast_from(0.19025925925925925925e-1_f64) * t28371 + F::cast_from(0.68493333333333333331e-1_f64) * t28375 - F::cast_from(0.34246666666666666665e-1_f64) * t28379 - F::cast_from(0.10274e0_f64) * t28383 + F::cast_from(0.10274e0_f64) * t28387 - F::cast_from(0.17123333333333333333e-1_f64) * t28391;
    (t29669, t29672, t29685)
}
