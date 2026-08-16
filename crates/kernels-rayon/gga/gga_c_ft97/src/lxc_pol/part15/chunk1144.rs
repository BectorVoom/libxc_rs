//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1144/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1144(t5064: f64, t68528: f64, t1168: f64, t80522: f64, t10079: f64, t10157: f64, t14127: f64, t18675: f64, t1901: f64, t21416: f64, t21499: f64, t242: f64, t2574: f64, t2606: f64, t265: f64, t3885: f64, t3891: f64, t446: f64, t4934: f64, t5053: f64, t5181: f64, t51882: f64, t65313: f64, t67881: f64, t762: f64, t88079: f64, t88098: f64, t88114: f64, t9808: f64) -> (f64, f64, f64) {
    let t89147 = t68528 * t5064;
    let t89179 = t80522 * t1168;
    let t89187 = 4.0_f64 * t446 * t242 * t89147 + 4.0_f64 * t446 * t2574 * t5181 * t4934 - 112.0_f64 / 81.0_f64 * t51882 - 8.0_f64 / 9.0_f64 * t1901 * t3891 * t65313 * t88079 - 16.0_f64 / 27.0_f64 * t67881 + 8.0_f64 * t446 * t10157 * t762 * t21416 * t1168 - 12.0_f64 * t446 * t10157 * t265 * t4934 * t5053 + 8.0_f64 / 3.0_f64 * t1901 * t2606 * t9808 * t88098 + 8.0_f64 / 3.0_f64 * t1901 * t10079 * t3885 * t88114 - 4.0_f64 / 3.0_f64 * t446 * t242 * t89179 - 8.0_f64 * t1901 * t14127 * t18675 * t21499;
    (t89147, t89179, t89187)
}
