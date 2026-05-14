//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 992/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk992<F: Float>(t1526: F, t3713: F, t9483: F, t42262: F, t4906: F, t3695: F, t17698: F, t52679: F, t13292: F, t13324: F, t13361: F, t13378: F, t13383: F, t13390: F, t13616: F, t14354: F, t14361: F, t2320: F, t3806: F, t42270: F, t42273: F, t42288: F, t42293: F, t42295: F, t42320: F) -> (F,) {
    let t69132 = t1526 * t9483 * t3713 / 18.0;
    let t69137 = t1526 * t42262 * t4906;
    let t69141 = t1526 * t9483 * t3695 / 18.0;
    let t69143 = t1526 * t52679 * t17698;
    let t69153 = -t42270 / 36.0 - t42273 / 27.0 + t42288 / 18.0 - t1526 * t2320 * t13378 / 12.0 - t1526 * t3806 * t13383 / 9.0 + t1526 * t2320 * t13361 / 6.0 - t69132 - t1526 * t2320 * t13390 / 6.0 + t69137 / 54.0 - t69141 - 7.0 / 18.0 * t69143 - t1526 * t2320 * t13324 / 12.0 + t1526 * t13616 * t13292 / 3.0 - t42293 + t42295 / 9.0 - t42320 / 12.0 + t14354 + t14361;
    (t69153,)
}
