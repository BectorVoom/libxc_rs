//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 964/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk964<F: Float>(t2253: F, t5450: F, t5454: F, t10845: F, t4965: F, t904: F, t17766: F, t4334: F, t14487: F, t17749: F, t17753: F, t2938: F, t5468: F) -> (F, F, F, F, F, F, F) {
    let t18900 = t2253 * t5450;
    let t18902 = t2253 * t5454;
    let t18905 = t10845 * t4965 * t904;
    let t18908 = t4334 * t17766;
    let t18911 = t14487 * t17749;
    let t18914 = t4334 * t17753;
    let t18917 = t2938 * t5468;
    (t18900, t18902, t18905, t18908, t18911, t18914, t18917)
}
