//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1294/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1294<F: Float>(t101064: F, t4947: F, t93508: F, t14447: F, t29010: F, t7703: F, t18476: F, t922: F, t100486: F, t100489: F, t100494: F, t100505: F, t100514: F, t100519: F, t14492: F, t19396: F, t93485: F, t95903: F, t95938: F) -> (F, F, F) {
    let t101250 = t4947 * t93508 * t101064;
    let t101264 = t7703 * t14447 * t29010;
    let t101271 = t4947 * t18476 * t922;
    let t101281 = -F::new(0.10297067901234567901e-3) * t101264 + F::new(0.18424382716049382715e-2) * t100486 - F::new(0.73697530864197530861e-2) * t100489 - F::new(0.22109259259259259259e-2) * t95903 + F::new(0.14739506172839506172e-2) * t100494 + F::new(0.23168402777777777778e-3) * t7703 * t101271 - F::new(0.16581944444444444444e-2) * t100505 - F::new(0.72079475308641975309e-3) * t7703 * t14492 * t93485 * t19396 + t95938 - F::new(0.22109259259259259259e-2) * t100514 - F::new(0.33163888888888888888e-2) * t100519;
    (t101250, t101271, t101281)
}
