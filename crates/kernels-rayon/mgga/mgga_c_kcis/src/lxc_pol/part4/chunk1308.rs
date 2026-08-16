//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1308/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1308(t16768: f64, t4158: f64, t4992: f64, t86: f64, t1489: f64, t167: f64, t4163: f64, t4162: f64, t1497: f64, t4171: f64, t4170: f64, t833: f64) -> (f64, f64, f64, f64, f64) {
    let t16769 = 0.22109259259259259258e-2_f64 * t16768;
    let t16771 = t86 * t4992 * t4158;
    let t16772 = t167 * t1489;
    let t16773 = t4163 * t16772;
    let t16774 = t4162 * t16773;
    let t16775 = t16771 * t16774;
    let t16777 = t167 * t1497;
    let t16778 = t4171 * t16777;
    let t16779 = t4170 * t16778;
    let t16780 = t16771 * t16779;
    let t16782 = t167 * t833;
    (t16769, t16771, t16775, t16780, t16782)
}
