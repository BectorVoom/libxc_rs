//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 993/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk993(t25: f64, t3533: f64, t1251: f64, t1259: f64, t2888: f64, t3490: f64, t3501: f64, t3500: f64, t3521: f64, t110: f64, t992: f64, t1254: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t11013 = t25 * t3533;
    let t11014 = t1251 * t11013;
    let t11020 = t2888 * t1259;
    let t11034 = t3490 * t3501;
    let t11041 = t3500 * t3521;
    let t11042 = t1251 * t11041;
    let t11061 = t110 * t992;
    let t11062 = t11061 * t1254;
    (t11014, t11020, t11034, t11042, t11061, t11062)
}
