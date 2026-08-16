//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2152/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2152(t52973: f64, t11820: f64, t5019: f64, t11791: f64, t5024: f64, t5002: f64, t11153: f64, t4899: f64, t3540: f64, t4961: f64, t1227: f64, t4973: f64, t49850: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t52974 = t52973 / 4608.0_f64;
    let t52987 = t5019 * t11820;
    let t52988 = t52987 / 864.0_f64;
    let t52991 = t5024 * t11791;
    let t52992 = t52991 / 1296.0_f64;
    let t52993 = t5002 * t11820;
    let t52994 = t52993 / 4608.0_f64;
    let t52995 = t4899 * t11153;
    let t52999 = t4961 * t3540;
    let t53000 = t52999 / 864.0_f64;
    let t53033 = t1227 * t49850 * t4973;
    (t52974, t52988, t52992, t52994, t52995, t53000, t53033)
}
