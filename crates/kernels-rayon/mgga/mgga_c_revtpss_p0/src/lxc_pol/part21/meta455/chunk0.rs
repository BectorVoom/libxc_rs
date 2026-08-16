//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1985/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1985(t2470: f64, t4480: f64, t2465: f64, t11008: f64, t1579: f64, t2771: f64, t1558: f64, t836: f64, t231: f64, t2797: f64, t2782: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t14485 = t4480 * t2470;
    let t14486 = t2465 * t14485;
    let t14489 = t11008 * t1579 * t2771;
    let t14494 = t1558 * t836;
    let t14495 = t14494 * t231;
    let t14496 = t2797 * t14495;
    let t14498 = 0.10975748638225852664e-1_f64 * t2782 * t14496;
    let t14502 = t860 * t1558;
    (t14485, t14486, t14489, t14494, t14495, t14496, t14498, t14502)
}
