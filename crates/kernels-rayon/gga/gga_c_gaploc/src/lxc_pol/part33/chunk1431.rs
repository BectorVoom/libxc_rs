//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1431/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1431(t1: f64, t39048: f64, t2021: f64, t2610: f64, t38912: f64, t12205: f64, t1858: f64, t2005: f64, t2028: f64, t28714: f64, t33501: f64, t33505: f64, t33508: f64, t33518: f64, t33522: f64, t33526: f64, t33529: f64, t33532: f64, t33536: f64, t33539: f64, t33544: f64, t33546: f64, t4820: f64, t5598: f64, t787: f64) -> (f64, f64) {
    let t39145 = t39048 * t1;
    let t39146 = t2021 * t39145;
    let t39149 = t2610 * t38912;
    let t39154 = -t33501 - t33505 - 0.79445533226334281486e-1_f64 * t787 * t1858 * t12205 * t2028 + 0.21450293971110256002e1_f64 * t39146 * t2005 + t33508 + t33518 + t33522 - t33526 - t33529 - 0.79445533226334281486e-1_f64 * t5598 * t4820 * t39149 - t33532 + t33536 + t33539 - t33544 - t33546 + 0.10224780254378866581e1_f64 * t28714;
    (t39149, t39154)
}
