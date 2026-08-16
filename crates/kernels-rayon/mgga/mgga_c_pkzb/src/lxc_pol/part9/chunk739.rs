//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 739/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk739(t5156: f64, t1508: f64, t1511: f64, t1536: f64, t1634: f64, t1816: f64, t2536: f64, t2718: f64, t4025: f64, t4996: f64, t5005: f64, t5130: f64, t5132: f64, t5134: f64, t5139: f64, t5141: f64, t5144: f64, t5148: f64, t5150: f64, t5154: f64) -> (f64, f64, f64, f64) {
    let t5157 = 0.17544670867903938621e1_f64 * t5156;
    let t5158 = t1511 * t1508;
    let t5159 = 0.51947577317044391276e2_f64 * t5158;
    let t5160 = 18.0_f64 * t1536 * t1634 * t2718 - 3.0_f64 * t1816 * t2536 * t4025 + t4996 + t5005 - t5130 + t5132 - t5134 - t5139 + t5141 - t5144 - t5148 + t5150 - t5154 - t5157 - t5159;
    (t5157, t5158, t5159, t5160)
}
