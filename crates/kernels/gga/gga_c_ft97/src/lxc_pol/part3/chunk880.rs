//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 880/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk880<F: Float>(t14895: F, t14951: F, t19004: F, t19008: F, t19013: F, t19018: F, t19022: F, t19025: F, t19028: F, t19032: F, t19243: F, t19246: F, t19249: F, t19252: F, t19255: F, t19258: F, t19261: F, t19265: F, t19269: F, t19754: F, t19757: F, t19761: F) -> (F, F) {
    let t19836 = -4.0 / 3.0 * t19004 + 4.0 / 9.0 * t19008 - 8.0 / 9.0 * t14895 + t14951 + t19013 / 3.0 - 2.0 / 3.0 * t19018 - 2.0 / 3.0 * t19022 - 2.0 * t19025 - 8.0 / 3.0 * t19028 + t19032 / 3.0 - t19243;
    let t19838 = t19246 / 3.0;
    let t19839 = 2.0 / 3.0 * t19249;
    let t19849 = t19838 - t19839 - t19754 / 4.0 - t19757 / 2.0 + 3.0 / 8.0 * t19761 + 2.0 / 3.0 * t19252 - 2.0 / 9.0 * t19255 - 10.0 / 27.0 * t19258 + 8.0 / 9.0 * t19261 + 2.0 / 3.0 * t19265 - 4.0 / 3.0 * t19269;
    (t19836, t19849)
}
