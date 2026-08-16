//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1431/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1431(t12251: f64, t1980: f64, t12161: f64, t296: f64, t1: f64, t787: f64, t12244: f64, t2028: f64, t28593: f64, t33368: f64, t33376: f64, t33381: f64, t33385: f64, t33387: f64, t33389: f64, t33392: f64, t33394: f64, t33397: f64, t33405: f64, t33409: f64, t5669: f64) -> (f64, f64) {
    let t39118 = t1980 * t12251;
    let t39121 = t296 * t12161;
    let t39123 = t787 * t39121 * t1;
    let t39126 = -t33368 + 0.1022478025437886658e1_f64 * t5669 * t12244 + t28593 - 0.79445533226334281486e-1_f64 * t39118 * t2028 - 0.79445533226334281486e-1_f64 * t39123 * t2028 + t33376 + t33381 - t33385 + t33387 - t33389 + t33392 + t33394 - t33397 - t33405 - t33409;
    (t39121, t39126)
}
