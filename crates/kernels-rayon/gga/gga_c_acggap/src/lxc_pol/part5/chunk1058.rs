//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1058/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1058(t1165: f64, t16020: f64, t3194: f64, t530: f64, t3409: f64, t5209: f64, t1456: f64, t3228: f64, t1462: f64, t1451: f64, t3237: f64, t4728: f64, t997: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18620 = t3194 * t1165 * t530 * t16020;
    let t18622 = t3409 * t5209;
    let t18628 = t3228 * t1456;
    let t18633 = t3228 * t1462;
    let t18647 = t3237 * t1451;
    let t18649 = t997 * t4728;
    (t18620, t18622, t18628, t18633, t18647, t18649)
}
