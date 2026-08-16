//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1355/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1355(t19456: f64, t8533: f64, t31772: f64, t4028: f64, t12725: f64, t33234: f64, t6525: f64, t1388: f64, t22574: f64, t26558: f64, t33357: f64, t33610: f64, t6876: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120924 = 2.0_f64 * t19456 * t8533;
    let t120926 = 2.0_f64 * t4028 * t31772;
    let t120928 = 2.0_f64 * t12725 * t8533;
    let t120930 = 2.0_f64 * t33234 * t6525;
    let t120940 = 6.0_f64 * t22574 * t26558 * t33357 * t1388;
    let t120941 = t6876 * t33610;
    (t120924, t120926, t120928, t120930, t120940, t120941)
}
