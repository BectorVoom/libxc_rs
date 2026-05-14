//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1047/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1047<F: Float>(t4215: F, t6360: F, t4223: F, t6340: F, t19873: F, t6332: F, t6331: F, t20879: F, t467: F, t492: F, t500: F, t19715: F, t499: F, t498: F, t1504: F, t2259: F, t4313: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20994 = t4215 * t6360;
    let t20996 = t4223 * t6340;
    let t20998 = t6332 * t19873;
    let t20999 = t6331 * t20998;
    let t21001 = t20879 * t467;
    let t21002 = t21001 * t492;
    let t21003 = t21002 * t500;
    let t21005 = t499 * t19715;
    let t21006 = t498 * t21005;
    let t21007 = t1504 * t21006;
    let t21009 = t2259 * t4313;
    (t20994, t20996, t20998, t20999, t21001, t21003, t21006, t21007, t21009)
}
