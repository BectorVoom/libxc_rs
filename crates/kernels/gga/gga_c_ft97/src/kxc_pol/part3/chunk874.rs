//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 874/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk874<F: Float>(t19668: F, t19729: F, t845: F, t91: F, t14708: F, t14711: F, t14715: F, t14718: F, t15081: F, t15082: F, t15083: F, t15089: F, t15096: F, t18999: F, t14895: F, t15118: F, t19004: F, t19008: F, t19013: F, t19018: F, t19022: F, t19025: F, t19028: F, t19032: F, t19243: F) -> (F, F, F) {
    let t19730 = t19668 + t19729;
    let t19732 = t91 * t845 * t19730;
    let t19737 = -t15081 - t15082 + t15083 - t15089 - t15096 - t14708 + t19732 / 6.0 - t14711 - 8.0 / 81.0 * t14715 - 4.0 / 27.0 * t14718 - 4.0 / 9.0 * t18999;
    let t19748 = -4.0 / 9.0 * t19004 + 4.0 / 27.0 * t19008 - 8.0 / 27.0 * t14895 + t15118 + t19013 / 9.0 - 2.0 / 9.0 * t19018 - 2.0 / 9.0 * t19022 - 2.0 / 3.0 * t19025 - 8.0 / 9.0 * t19028 + t19032 / 9.0 - t19243 / 3.0;
    (t19732, t19737, t19748)
}
