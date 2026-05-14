//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 504/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk504<F: Float>(t140: F, t3404: F, t550: F, t133: F, t1010: F, t1015: F, t2001: F, t3348: F, t3350: F, t3356: F, t3381: F, t3384: F, t3387: F, t3392: F, t3394: F, t399: F) -> (F, F, F) {
    let t141 = 0.1e-59 < t140;
    let t3405 = t550 * t3404;
    let t3406 = t133 * t3405;
    let t3408 = piecewise3(t141, 2.0 * t3348 - 0.1208182677680765956e1 * t3350 * t399 + 0.1208182677680765956e1 * t1010 * t399 - 2.0 * t2001 * t3356 + 2.0 * t3381 - 2.0 * t2001 * t3384 + 0.60409133884038297798e0 * t3387 * t399 - 0.60409133884038297798e0 * t1015 * t399 + 2.0 * t3392 * t3394 - t3406, 0.0);
    (t3405, t3406, t3408)
}
