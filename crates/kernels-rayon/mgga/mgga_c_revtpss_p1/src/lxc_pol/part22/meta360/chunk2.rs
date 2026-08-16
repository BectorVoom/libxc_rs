//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1880/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1880(t3478: f64, t434: f64, t1179: f64, t3488: f64, t1175: f64, t3520: f64, t3519: f64, t444: f64) -> (f64, f64, f64, f64) {
    let t12472 = 1.0_f64 / t3478 / t434;
    let t12476 = t3488 * t1179;
    let t12481 = t1175 * t3520;
    let t12485 = 1.0_f64 / t3519 / t444;
    (t12472, t12476, t12481, t12485)
}
