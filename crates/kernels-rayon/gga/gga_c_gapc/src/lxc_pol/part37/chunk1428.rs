//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1428/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1428(t12590: f64, t4908: f64, t33212: f64, t33217: f64, t33228: f64, t36508: f64, t36510: f64, t36512: f64, t36513: f64, t36515: f64, t36516: f64, t36517: f64, t36518: f64) -> (f64, f64) {
    let t38710 = 4.0_f64 * t4908 * t12590;
    let t38714 = t36508 + 0.36231816839129402172e-6_f64 * t33212 - t36510 + 0.18115908419564701086e-6_f64 * t33217 + t36512 + t36513 - 0.25301106770833333334e-5_f64 * t33228 + t36515 + t36516 - t36517 - t36518;
    (t38710, t38714)
}
