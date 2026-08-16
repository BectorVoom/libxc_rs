//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 558/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk558(t4180: f64, t4181: f64, t4182: f64, t1512: f64, t2639: f64, t249: f64, t2571: f64, t2602: f64, t2603: f64, t2618: f64, t4152: f64, t4155: f64, t4159: f64, t4163: f64, t4167: f64, t4170: f64, t4172: f64, t4178: f64, t787: f64, t831: f64, t849: f64) -> (f64, f64) {
    let t4184 = t4180 * t4181 * t4182;
    let t4187 = t2639 * t1512;
    let t4189 = t2602 + 7.0_f64 / 144.0_f64 * t2603 + 7.0_f64 / 144.0_f64 * t4152 + t2571 * t4155 / 16.0_f64 - t787 * t4159 / 48.0_f64 + t4163 * t249 / 3072.0_f64 - t4167 * t831 / 3072.0_f64 - 7.0_f64 / 4608.0_f64 * t4170 - t4172 * t849 / 768.0_f64 - t2618 * t1512 / 3072.0_f64 + t4178 * t4184 / 1536.0_f64 + 7.0_f64 / 4608.0_f64 * t4187;
    (t4184, t4189)
}
