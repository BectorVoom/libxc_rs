//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1168/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1168<F: Float>(t2001: F, t5561: F, t5946: F, t1755: F, t30644: F, t5792: F, t7822: F, t13287: F, t31443: F, t39858: F, t2297: F, t8406: F) -> (F, F, F, F, F, F) {
    let t40105 = t2001 * t5561;
    let t40107 = t2001 * t5946;
    let t40109 = t30644 * t1755;
    let t40111 = t7822 * t5792;
    let t40114 = t31443 * t13287 * t39858;
    let t40116 = t2297 * t8406;
    (t40105, t40107, t40109, t40111, t40114, t40116)
}
