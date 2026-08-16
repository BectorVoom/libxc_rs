//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2024/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2024(t28: f64, t12000: f64, t1649: f64, t2: f64, t3711: f64, t1302: f64, t15956: f64, t16: f64, t3231: f64, t3673: f64, t5178: f64, t5181: f64, t584: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t16003 = t12000 * t1649;
    let t16006 = t3711 * t2;
    let t16016 = piecewise3(t29, 0.0_f64, 8.0_f64 / 27.0_f64 * t16003 * t3673 + 8.0_f64 / 9.0_f64 * t16006 * t15956 - 2.0_f64 / 9.0_f64 * t5178 * t3231 - 4.0_f64 / 3.0_f64 * t1302 * t584 + 4.0_f64 * t5181 * t16);
    (t16003, t16016)
}
