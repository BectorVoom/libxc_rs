//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1138/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1138(t30780: f64, t36209: f64, t1460: f64, t30148: f64, t2035: f64, t7323: f64, t16314: f64, t336: f64, t570: f64, t4264: f64, t7436: f64, t142: f64, t3706: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36210 = t30780 * t36209;
    let t36213 = t30148 * t1460;
    let t36214 = t2035 * t7323 * t36213;
    let t36217 = t570 * t336 * t16314;
    let t36220 = t7436 * t4264;
    let t36222 = t142 * t3706;
    (t36210, t36213, t36214, t36217, t36220, t36222)
}
