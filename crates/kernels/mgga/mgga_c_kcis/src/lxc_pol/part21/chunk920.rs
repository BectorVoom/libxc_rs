//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 920/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk920<F: Float>(t1220: F, t15422: F, t13714: F, t10945: F, t13710: F, t13712: F, t13717: F, t13720: F, t13723: F, t13726: F, t13729: F, t13732: F, t13735: F, t13738: F, t13742: F, t9681: F, t9683: F, t9691: F, t9700: F) -> (F, F) {
    let t15423 = t15422 * t1220;
    let t15432 = 0.2283111111111111111e-1 * t13714;
    let t15442 = -t10945 - 0.1522074074074074074e-1 * t9691 + 0.38051851851851851851e-2 * t9683 - 0.11415555555555555555e-1 * t9700 + 0.57077777777777777777e-2 * t9681 - 0.76103703703703703702e-2 * t13710 + 0.76103703703703703701e-2 * t13712 - t15432 + 0.1255711111111111111e0 * t13717 - 0.19025925925925925925e-1 * t13720 + 0.68493333333333333331e-1 * t13723 - 0.45662222222222222221e-1 * t13726 - 0.11415555555555555555e-1 * t13729 - 0.10274e0 * t13732 + 0.13698666666666666666e0 * t13735 + 0.34246666666666666666e-1 * t13738 - 0.34246666666666666666e-1 * t13742;
    (t15423, t15442)
}
