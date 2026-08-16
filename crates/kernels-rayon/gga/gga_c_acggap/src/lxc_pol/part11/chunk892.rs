//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 892/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk892(t1967: f64, t7763: f64, t7701: f64, t381: f64, t7636: f64, t7461: f64, t7637: f64, t7770: f64, t13716: f64, t577: f64, t584: f64, t1072: f64, t167: f64, t7322: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t30584 = t1967 * t7763;
    let t30586 = t1967 * t7701;
    let t30589 = t381 * t7636;
    let t30590 = t30589 * t7461;
    let t30591 = 0.28582678745379824649e-2_f64 * t30590;
    let t30592 = t7637 * t7770;
    let t30594 = t13716 * t577;
    let t30595 = t30594 * t584;
    let t30596 = 0.37042881944444444445e0_f64 * t30595;
    let t30598 = t7322 * t167 * t1072;
    (t30584, t30586, t30589, t30591, t30592, t30594, t30596, t30598)
}
