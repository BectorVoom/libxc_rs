//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 858/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk858<F: Float>(t13593: F, t5676: F, t11576: F, t2033: F, t2365: F, t2610: F, t44712: F, t701: F, t6066: F, t7630: F, t36635: F, t959: F) -> (F, F, F, F, F) {
    let t45299 = t5676 * t13593;
    let t45300 = F::cast_from(0.14896037479937677779e-1_f64) * t45299;
    let t45303 = t2033 * t2365 * t2610 * t11576;
    let t45304 = F::cast_from(0.14896037479937677779e-1_f64) * t45303;
    let t45305 = t44712 * t701;
    let t45308 = F::cast_from(0.71500979903700853338e0_f64) * t7630 * t6066 * t45305;
    let t45314 = t36635 * t959;
    (t45300, t45304, t45305, t45308, t45314)
}
