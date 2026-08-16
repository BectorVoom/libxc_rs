//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2308/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2308(t58021: f64, t46278: f64, t1484: f64, t4303: f64, t16634: f64, t4205: f64, t40738: f64, t40754: f64, t12895: f64, t2522: f64, t40741: f64, t40743: f64, t40748: f64, t40760: f64, t4307: f64, t5544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67162 = 0.17544670867903938621e1_f64 * t58021;
    let t67163 = 0.48796115851357829289e-1_f64 * t46278;
    let t67164 = t1484 * t4303;
    let t67169 = 12.0_f64 * t4205 * t16634;
    let t67170 = 0.21687162600603479684e-1_f64 * t40738;
    let t67174 = 0.10389515463408878255e3_f64 * t40754;
    let t67175 = 9.0_f64 * t12895 * t2522 * t5544 - 18.0_f64 * t2522 * t4307 * t67164 - t40741 - t40743 + t40748 + t40760 - t67162 + t67163 + t67169 - t67170 + t67174;
    (t67162, t67163, t67169, t67170, t67174, t67175)
}
