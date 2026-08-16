//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 969/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk969(t1459: f64, t15167: f64, t3018: f64, t1460: f64, t5186: f64, t2993: f64, t1484: f64, t5218: f64, t1483: f64, t15374: f64, t1472: f64, t5154: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t17748 = t15167 * t1459;
    let t17750 = 0.48245472966453314466e2_f64 * t3018 * t17748;
    let t17751 = t1460 * t5186;
    let t17753 = 6.0_f64 * t2993 * t17751;
    let t17755 = t1484 * t5218;
    let t17758 = t15374 * t1483;
    let t17761 = t1472 * t5154;
    (t17748, t17750, t17751, t17753, t17755, t17758, t17761)
}
