//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1040/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1040(t13131: f64, t13132: f64, t3210: f64, t13130: f64, t1804: f64, t2850: f64, t3200: f64, t3045: f64, t4781: f64, t1646: f64, t2635: f64, t4546: f64) -> (f64, f64, f64, f64, f64) {
    let t13133 = t13131 * t13132;
    let t13134 = t3210 * t13133;
    let t13135 = t13130 * t13134;
    let t13137 = t1804 * t2850;
    let t13138 = t3210 * t13137;
    let t13139 = t3200 * t13138;
    let t13145 = t4781 * t3045;
    let t13150 = t1646 * t2635;
    let t13151 = t4546 * t13150;
    (t13135, t13139, t13145, t13150, t13151)
}
