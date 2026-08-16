//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3207/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3207(t11888: f64, t11914: f64, t1216: f64, t14992: f64, t15032: f64, t15245: f64, t15429: f64, t15772: f64, t1729: f64, t19145: f64, t19153: f64, t19156: f64, t19169: f64, t19170: f64, t3565: f64, t3604: f64, t3610: f64, t45323: f64, t4964: f64, t5011: f64, t5068: f64, t5076: f64, t5086: f64, t52480: f64, t53545: f64, t6256: f64, t6260: f64, t6263: f64, t6265: f64) -> f64 {
    let t66769 = 4.0_f64 * t1216 * t5011 * t52480 * t53545 - 12.0_f64 * t11888 * t19145 * t19156 + 2.0_f64 * t11914 * t15429 * t6256 + t11914 * t15429 * t6260 + 2.0_f64 * t11914 * t19145 * t19153 + 8.0_f64 * t19169 * t3610 * t5068 - 4.0_f64 * t14992 * t15245 + 4.0_f64 * t15032 * t5076 + 2.0_f64 * t15772 * t1729 + 4.0_f64 * t19170 * t3604 + t3565 * t6265 - t45323 * t6263 + 4.0_f64 * t4964 * t5086;
    t66769
}
