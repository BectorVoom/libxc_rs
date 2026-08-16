//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 897/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk897<F: Float>(t43446: F, t43454: F, t2639: F, t3614: F, t7284: F, t787: F, t13593: F, t5676: F, t11576: F, t2033: F, t2365: F, t2610: F) -> (F, F, F, F, F) {
    let t45287 = F::cast_from(0.41708904943825497782e0_f64) * t43446;
    let t45288 = F::cast_from(0.35750489951850426669e0_f64) * t43454;
    let t45298 = F::cast_from(0.25025342966295298669e1_f64) * t787 * t7284 * t3614 * t2639;
    let t45299 = t5676 * t13593;
    let t45300 = F::cast_from(0.14896037479937677779e-1_f64) * t45299;
    let t45303 = t2033 * t2365 * t2610 * t11576;
    (t45287, t45288, t45298, t45300, t45303)
}
