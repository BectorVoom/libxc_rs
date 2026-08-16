//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 914/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk914(t1168: f64, t2373: f64, t2574: f64, t762: f64, t2569: f64, t10052: f64, t242: f64, t10085: f64, t3898: f64, t11593: f64, t14095: f64, t14100: f64, t14105: f64, t14110: f64, t14114: f64, t14118: f64, t14122: f64, t14126: f64, t14130: f64, t14135: f64, t14138: f64, t1901: f64, t446: f64, t9982: f64) -> (f64, f64) {
    let t14140 = t1168 * t2373;
    let t14142 = t2574 * t762 * t14140;
    let t14145 = t1168 * t2569;
    let t14146 = t10052 * t14145;
    let t14147 = t242 * t14146;
    let t14150 = t10085 * t3898;
    let t14153 = 2.0_f64 / 9.0_f64 * t1901 * t14095 + 4.0_f64 / 9.0_f64 * t1901 * t14100 + t1901 * t14105 / 9.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t14110 + 4.0_f64 / 27.0_f64 * t14114 - t9982 - 8.0_f64 / 9.0_f64 * t11593 * t14118 + 8.0_f64 / 27.0_f64 * t11593 * t14122 - t14126 - 4.0_f64 / 3.0_f64 * t1901 * t14130 - 2.0_f64 * t446 * t14135 - 22.0_f64 / 27.0_f64 * t14138 - 2.0_f64 / 3.0_f64 * t446 * t14142 - 2.0_f64 * t446 * t14147 + 2.0_f64 / 9.0_f64 * t1901 * t14150;
    (t14146, t14153)
}
