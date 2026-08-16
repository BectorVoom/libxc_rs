//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 802/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk802(t1023: f64, t4999: f64, t1020: f64, t1714: f64, t2822: f64, t1662: f64, t2855: f64, t1021: f64, t1774: f64, t2825: f64, t1092: f64, t1773: f64, t3182: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5000 = t4999 * t1023;
    let t5001 = t1020 * t5000;
    let t5003 = t2822 * t1714;
    let t5005 = t2855 * t1662;
    let t5006 = t1021 * t5005;
    let t5007 = t1020 * t5006;
    let t5010 = t2825 * t1774;
    let t5011 = t1092 * t5010;
    let t5013 = t3182 * t1773;
    (t5000, t5001, t5003, t5005, t5006, t5007, t5010, t5011, t5013)
}
