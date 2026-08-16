//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 663/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk663(t26: f64, t9555: f64, t1771: f64, t685: f64, t2406: f64, t458: f64, t2410: f64, t2414: f64, t322: f64, t668: f64, t17: f64, t2346: f64, t667: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9556 = t26 * t9555;
    let t9557 = 28.0_f64 / 27.0_f64 * t9556;
    let t9558 = t1771 * t685;
    let t9560 = t458 * t2406;
    let t9562 = t458 * t2410;
    let t9564 = t458 * t2414;
    let t9567 = 1.0_f64 / t322 / t668;
    let t9568 = t17 * t9567;
    let t9570 = 1.0_f64 / t2346 / t667;
    (t9556, t9557, t9558, t9560, t9562, t9564, t9567, t9568, t9570)
}
