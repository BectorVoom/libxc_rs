//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 665/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk665(t191: f64, t2999: f64, t26: f64, t1771: f64, t685: f64, t322: f64, t668: f64, t17: f64, t2346: f64, t667: f64, t113: f64, t170: f64, t7512: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9555 = t2999 * t191;
    let t9556 = t26 * t9555;
    let t9557 = 28.0_f64 / 27.0_f64 * t9556;
    let t9558 = t1771 * t685;
    let t9567 = 1.0_f64 / t322 / t668;
    let t9568 = t17 * t9567;
    let t9570 = 1.0_f64 / t2346 / t667;
    let t9577 = 1.0_f64 / t2346 / t113;
    let t9606 = 4.0_f64 * t170 * t7512;
    (t9555, t9556, t9557, t9558, t9567, t9568, t9570, t9577, t9606)
}
