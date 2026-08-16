//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 903/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk903(t13746: f64, t13753: f64, t13728: f64, t13732: f64, t13736: f64, t13743: f64, t13750: f64, t13759: f64, t13981: f64, t9872: f64, t9876: f64, t13780: f64) -> (f64, f64) {
    let t13983 = 4.0_f64 / 3.0_f64 * t13746;
    let t13984 = 2.0_f64 / 3.0_f64 * t13753;
    let t13986 = 4.0_f64 * t13728 - 22.0_f64 / 9.0_f64 * t13732 + 2.0_f64 / 3.0_f64 * t13736 - t13981 + 2.0_f64 * t13743 - t13983 - t13750 + t13984 - t9872 - t9876 - 4.0_f64 / 3.0_f64 * t13759;
    let t13993 = 2.0_f64 / 9.0_f64 * t13780;
    (t13986, t13993)
}
