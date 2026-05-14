//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 842/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk842<F: Float>(t1755: F, t5507: F, t28: F, t89: F, t376: F, t5696: F, t432: F, t473: F) -> (F, F, F, F) {
    let t23043 = t5507 * t1755;
    let t23044 = t28 * t23043;
    let t23045 = t89 * t23044;
    let t23047 = t376 * t5696;
    let t23048 = t89 * t23047;
    let t23050 = t473 * t432;
    (t23043, t23045, t23048, t23050)
}
