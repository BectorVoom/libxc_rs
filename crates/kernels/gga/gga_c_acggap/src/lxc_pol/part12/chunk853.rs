//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 853/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk853<F: Float>(t1077: F, t1083: F, t1980: F, t355: F, t7458: F, t31190: F, t5011: F, t30727: F, t7670: F, t7676: F, t7724: F, t1131: F, t1983: F, t2095: F, t2056: F, t7600: F) -> (F, F, F, F, F, F) {
    let t31464 = t1980 * t7458 * t1083 * t355 * t1077;
    let t31468 = t1980 * t7458 * t5011 * t31190;
    let t31470 = t30727 * t7670;
    let t31472 = t7676 * t7724;
    let t31475 = t2095 * t1983 * t1131;
    let t31477 = t7600 * t2056;
    (t31464, t31468, t31470, t31472, t31475, t31477)
}
