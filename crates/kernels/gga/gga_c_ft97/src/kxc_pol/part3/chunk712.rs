//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 712/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk712<F: Float>(t1882: F, t4569: F, t4595: F, t11897: F, t11913: F, t11981: F, t11999: F, t12002: F, t16482: F, t16486: F, t16490: F, t16535: F, t1901: F, t28: F, t446: F, t8475: F, t8485: F, t8516: F, t8534: F, t89: F) -> (F,) {
    let t16539 = t1882 * t4569;
    let t16541 = t1882 * t4595;
    let t16544 = -4.0 / 27.0 * t8475 - 4.0 / 27.0 * t8485 - t446 * t16482 / 3.0 - t11897 - 2.0 / 9.0 * t1901 * t16486 - t11913 + 4.0 / 27.0 * t8516 - t8534 - 2.0 / 9.0 * t16490 + t89 * t28 * t16535 / 3.0 - t11981 - 2.0 / 9.0 * t16539 + 2.0 / 9.0 * t16541 - t11999 + 4.0 / 27.0 * t12002;
    (t16544,)
}
