//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1096/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1096(t1035: f64, t13786: f64, t3061: f64, t1045: f64, t4547: f64, t1680: f64, t2980: f64, t2938: f64, t2939: f64, t4722: f64, t9660: f64, t2988: f64, t4718: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13787 = t1035 * t13786;
    let t13790 = t3061 * t1035;
    let t13791 = t4547 * t1045;
    let t13796 = t1680 * t2980;
    let t13798 = 2.0_f64 * t2938 * t13796;
    let t13799 = t4722 * t2939;
    let t13801 = 0.96490945932906628932e2_f64 * t9660 * t13799;
    let t13802 = t4718 * t2988;
    (t13787, t13790, t13791, t13798, t13801, t13802)
}
