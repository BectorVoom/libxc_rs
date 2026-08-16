//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1047/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1047(t3255: f64, t5432: f64, t5436: f64, t5442: f64, t11671: f64, t544: f64, t5428: f64, t5454: f64, t518: f64, t5457: f64, t5490: f64, t1098: f64, t5528: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16543 = 0.13140859333333333334e-2_f64 * t3255 * t5432;
    let t16545 = 0.8760572888888888889e-3_f64 * t3255 * t5436;
    let t16547 = 0.17521145777777777778e-2_f64 * t3255 * t5442;
    let t16552 = t11671 * t544;
    let t16562 = 0.14600954814814814815e-2_f64 * t3255 * t5428;
    let t16567 = 0.13140859333333333333e-2_f64 * t3255 * t5454;
    let t16582 = t5457 * t518;
    let t16587 = t3255 * t5490;
    let t16601 = 0.13140859333333333333e-2_f64 * t1098 * t5528;
    (t16543, t16545, t16547, t16552, t16562, t16567, t16582, t16587, t16601)
}
