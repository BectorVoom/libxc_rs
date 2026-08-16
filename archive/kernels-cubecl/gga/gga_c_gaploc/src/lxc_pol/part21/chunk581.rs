//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 581/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk581<F: Float>(t3248: F, t943: F, t2673: F, t959: F, t2610: F, t935: F, t2365: F, t2033: F, t123: F, t883: F) -> (F, F, F, F, F, F, F) {
    let t3250 = F::cast_from(0.64087718584518535698e-3_f64) * t943 * t3248;
    let t3275 = F::cast_from(0.29792074959875355558e-1_f64) * t2673 * t959;
    let t3280 = t2610 * t935;
    let t3281 = t2365 * t3280;
    let t3283 = F::cast_from(0.29792074959875355558e-1_f64) * t2033 * t3281;
    let t3294 = t935 * t123;
    let t3295 = t3294 * t883;
    (t3250, t3275, t3280, t3281, t3283, t3294, t3295)
}
