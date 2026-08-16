//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1292/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1292(t3255: f64, t5428: f64, t16069: f64, t5425: f64, t5454: f64, t531: f64, t5481: f64, t833: f64, t3761: f64, t2645: f64, t5452: f64, t1897: f64, t3754: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16562 = 0.14600954814814814815e-2_f64 * t3255 * t5428;
    let t16563 = t5425 * t16069;
    let t16567 = 0.13140859333333333333e-2_f64 * t3255 * t5454;
    let t16568 = t5481 * t531;
    let t16569 = t16568 * t833;
    let t16570 = t3761 * t16569;
    let t16574 = t3761 * t5452 * t2645;
    let t16577 = t1897 * t3754;
    (t16562, t16563, t16567, t16570, t16574, t16577)
}
