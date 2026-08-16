//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1371/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1371(t22574: f64, t25988: f64, t36740: f64, t26168: f64, t8607: f64, t31747: f64, t4028: f64, t121159: f64, t121160: f64, t121162: f64, t121165: f64, t121169: f64, t1849: f64, t25965: f64, t26977: f64, t27147: f64, t31246: f64, t31532: f64, t31722: f64, t4077: f64, t6517: f64, t7042: f64, t7472: f64, t7941: f64) -> f64 {
    let t121174 = 3.0_f64 * t22574 * t36740 * t25988;
    let t121177 = 3.0_f64 * t8607 * t26168;
    let t121179 = 2.0_f64 * t4028 * t31747;
    let t121180 = t1849 * t31722 - 2.0_f64 * t25965 * t7042 - 2.0_f64 * t26977 * t7472 - 2.0_f64 * t27147 * t6517 + t31246 * t7941 - 2.0_f64 * t31532 * t4077 - t121159 + t121160 - t121162 - t121165 - t121169 - t121174 + t121177 - t121179;
    t121180
}
