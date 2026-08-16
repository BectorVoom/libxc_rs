//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 844/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk844(t333: f64, t3724: f64, t12884: f64, t12885: f64, t3716: f64, t3722: f64, t5794: f64, t1210: f64, t3696: f64, t4475: f64, t1171: f64, t3631: f64) -> (f64, f64, f64, f64, f64) {
    let t12888 = 1.0_f64 / t3724 / t333;
    let t12889 = t12884 * t12885 * t12888;
    let t12893 = t3722 * t3716 * t5794;
    let t12896 = t3696 * t1210;
    let t12897 = t12896 * t4475;
    let t12900 = t3631 * t1171;
    (t12888, t12889, t12893, t12897, t12900)
}
