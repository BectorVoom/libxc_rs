//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 901/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk901(t3178: f64, t7647: f64, t3171: f64, t7361: f64, t7839: f64, t1145: f64, t7329: f64, t1117: f64, t1103: f64, t7736: f64, t1089: f64, t429: f64, t7553: f64, t7554: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30750 = t7647 * t3178;
    let t30756 = t7647 * t3171;
    let t30758 = t7839 * t7361;
    let t30763 = t7329 * t1145;
    let t30767 = t7329 * t1117;
    let t30769 = t7736 * t1103;
    let t30773 = t7553 * t1089 * t429 * t7554;
    (t30750, t30756, t30758, t30763, t30767, t30769, t30773)
}
