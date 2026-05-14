//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 579/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk579<F: Float>(t10157: F, t3837: F, t6003: F, t1091: F, t24240: F, t2354: F, t24245: F, t1402: F, t3051: F, t3746: F, t6744: F, t92: F, t1173: F, t1424: F, t684: F, t6907: F, t761: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27997 = t10157 * t6003 * t3837;
    let t28001 = t24240 * t1091;
    let t28002 = t2354 * t28001;
    let t28006 = t2354 * t24245 * t1091;
    let t28010 = t1402 * t3051;
    let t28012 = t2354 * t6003 * t3746;
    let t28015 = t6744 * t92;
    let t28018 = t1424 * t1173;
    let t28020 = t2354 * t28018 * t684;
    let t28023 = t6907 * t761;
    (t27997, t28001, t28002, t28006, t28010, t28012, t28015, t28020, t28023)
}
