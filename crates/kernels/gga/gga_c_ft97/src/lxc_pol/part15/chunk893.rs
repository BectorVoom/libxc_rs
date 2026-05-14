//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 893/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk893<F: Float>(t20022: F, t942: F, t1901: F, t1902: F, t1903: F, t1909: F, t20044: F, t20098: F, t3194: F, t446: F, t452: F, t4612: F, t59506: F, t75048: F, t75050: F, t75071: F, t75115: F, t75117: F, t75119: F, t8210: F, t85393: F, t986: F) -> (F, F) {
    let t85740 = t20022 * t942;
    let t85752 = -4.0 / 3.0 * t446 * t452 * t986 * t20098 - 8.0 / 27.0 * t75048 - 4.0 / 9.0 * t75050 + 4.0 / 3.0 * t75071 - 8.0 / 9.0 * t75115 + 8.0 / 9.0 * t75117 + 8.0 / 3.0 * t75119 + 4.0 / 9.0 * t1901 * t1902 * t1903 * t20044 * t942 + 8.0 / 3.0 * t1901 * t1902 * t3194 * t85740 + 8.0 / 3.0 * t1901 * t1909 * t8210 * t85393 + 4.0 / 3.0 * t1901 * t59506 * t4612;
    (t85740, t85752)
}
