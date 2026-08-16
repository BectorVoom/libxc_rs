//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 659/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk659(t456: f64, t6069: f64, t1928: f64, t377: f64, t1931: f64, t1251: f64, t1844: f64, t4237: f64, t525: f64, t1815: f64, t2929: f64, t1934: f64, t310: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t6495 = t456 * t6069;
    let t6498 = t377 * t1928;
    let t6501 = t377 * t1931;
    let t6503 = t1251 * t1844;
    let t6507 = t4237 * t525;
    let t6510 = t2929 * t1815;
    let t6513 = t310 * t1934;
    (t6495, t6498, t6501, t6503, t6507, t6510, t6513)
}
