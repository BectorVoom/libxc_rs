//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 450/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk450<F: Float>(t4714: F, t526: F, t27: F, t89: F, t1957: F, t3530: F, t3535: F, t4654: F, t4658: F, t4662: F, t4666: F, t4671: F, t515: F, t1053: F) -> (F, F, F, F, F) {
    let t4715 = t526 * t4714;
    let t4717 = t89 * t27 * t4715;
    let t4719 = t1957 + t3530 + t3535 - t4654 / 27.0 + t4658 / 9.0 + t4662 / 9.0 - t4666 / 18.0 + t4671 / 3.0 - t4717 / 6.0;
    let t4720 = t515 * t4719;
    let t4724 = t1053 * t1053;
    (t4715, t4717, t4719, t4720, t4724)
}
