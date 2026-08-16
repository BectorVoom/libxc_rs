//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 908/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk908(t14627: f64, t19577: f64, t2855: f64, t6326: f64, t1021: f64, t2842: f64, t1022: f64, t18681: f64, t1020: f64, t1133: f64, t6496: f64, t9546: f64) -> (f64, f64, f64, f64, f64) {
    let t19578 = t14627 * t19577;
    let t19580 = t2855 * t6326;
    let t19581 = t1021 * t19580;
    let t19582 = t2842 * t19581;
    let t19584 = t1022 * t18681;
    let t19585 = t1021 * t19584;
    let t19586 = t1020 * t19585;
    let t19588 = t6496 * t1133;
    let t19589 = t9546 * t19588;
    (t19578, t19582, t19586, t19588, t19589)
}
