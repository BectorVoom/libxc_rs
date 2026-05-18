//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 311/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk311<F: Float>(t2211: F, t3429: F, t2210: F, t1570: F, t160: F, t3188: F, t157: F, t2097: F, t1557: F, t1017: F, t379: F, t2221: F) -> (F, F, F, F, F, F, F, F) {
    let t3430 = t2211 * t3429;
    let t3431 = t2210 * t3430;
    let t3434 = t160 * t1570;
    let t3435 = t3434 * t3188;
    let t3436 = t2210 * t3435;
    let t3439 = t2097 * t157;
    let t3440 = t160 * t1557;
    let t3441 = t3440 * t3188;
    let t3442 = t3439 * t3441;
    let t3445 = t160 * t1017;
    let t3446 = t3445 * t379;
    let t3447 = t2221 * t3446;
    (t3430, t3431, t3435, t3436, t3441, t3442, t3446, t3447)
}
