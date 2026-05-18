//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 909/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk909<F: Float>(t16579: F, t704: F, t420: F, t701: F, t17766: F, t3806: F, t13605: F, t17749: F, t13609: F, t17753: F, t17744: F, t2320: F) -> (F, F, F, F, F) {
    let t18049 = t704 * t16579;
    let t18050 = t420 * t18049;
    let t18051 = t701 * t18050;
    let t18054 = t3806 * t17766;
    let t18055 = t701 * t18054;
    let t18057 = t13605 * t17749;
    let t18058 = t701 * t18057;
    let t18060 = t13609 * t17753;
    let t18061 = t701 * t18060;
    let t18063 = t2320 * t17744;
    (t18051, t18055, t18058, t18061, t18063)
}
