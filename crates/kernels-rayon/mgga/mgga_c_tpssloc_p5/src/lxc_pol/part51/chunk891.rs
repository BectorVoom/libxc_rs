//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 891/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk891(t3941: f64, t8657: f64, t2039: f64, t577: f64, t7010: f64, t8508: f64, t8646: f64, t8654: f64, t192: f64, t533: f64) -> (f64, f64) {
    let t8659 = 27.0_f64 * t3941 * t8657;
    let t8660 = 0.45e1_f64 * t8646 * t577 + t8654 + 0.135e2_f64 * t7010 * t2039 + t8659 + t8508;
    let t8944 = t192 * t533;
    (t8660, t8944)
}
