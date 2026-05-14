//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 907/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk907<F: Float>(t13036: F, t13051: F, t11249: F, t3603: F, t13044: F, t1042: F, t13032: F, t3609: F, t1244: F, t13040: F, t471: F, t1032: F, t3552: F, t1246: F, t1250: F, t12732: F, t482: F) -> (F, F, F, F, F, F, F) {
    let t13052 = t13036 * t13051;
    let t13053 = t11249 * t3603;
    let t13054 = t13044 * t13053;
    let t13055 = t1042 * t13054;
    let t13058 = t13032 * t3609;
    let t13061 = t1244 * t13040;
    let t13062 = t13036 * t13061;
    let t13063 = t11249 * t471;
    let t13064 = t13044 * t13063;
    let t13065 = t1042 * t13064;
    let t13068 = t3552 * t1032;
    let t13069 = t13068 * t1246;
    let t13075 = t482 * t12732 * t1250;
    (t13052, t13055, t13058, t13062, t13065, t13069, t13075)
}
