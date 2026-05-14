//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 731/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk731<F: Float>(t5446: F, t8675: F, t1268: F, t2923: F, t3746: F, t4969: F, t904: F, t17744: F, t4342: F, t17780: F, t4973: F, t17727: F, t2253: F, t5450: F, t5454: F, t10845: F, t4965: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18877 = t8675 * t5446;
    let t18880 = t2923 * t3746 * t1268;
    let t18884 = t2923 * t4969 * t904;
    let t18887 = t4342 * t17744;
    let t18889 = t4342 * t17780;
    let t18893 = t2923 * t4973 * t904;
    let t18896 = t4342 * t17727;
    let t18900 = t2253 * t5450;
    let t18902 = t2253 * t5454;
    let t18905 = t10845 * t4965 * t904;
    (t18877, t18880, t18884, t18887, t18889, t18893, t18896, t18900, t18902, t18905)
}
