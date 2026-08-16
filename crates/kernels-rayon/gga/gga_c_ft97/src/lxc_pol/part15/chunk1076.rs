//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1076/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1076(t605: f64, t87113: f64, t87128: f64, t87144: f64, t87160: f64, t40530: f64, t62364: f64, t62410: f64, t86942: f64, t86946: f64, t86950: f64, t86954: f64, t86958: f64, t86962: f64, t86966: f64, t86970: f64, t86975: f64, t86979: f64) -> (f64, f64) {
    let t87163 = t605 * (t87113 + t87128 + t87144 + t87160);
    let t87175 = t62364 + t40530 - 6.0_f64 * t86942 + 4.0_f64 / 3.0_f64 * t86946 - 40.0_f64 / 243.0_f64 * t86950 - 4.0_f64 / 3.0_f64 * t86954 - t86958 / 18.0_f64 + 4.0_f64 / 3.0_f64 * t86962 + t86966 / 3.0_f64 - t86970 / 9.0_f64 + t62410 - 4.0_f64 / 3.0_f64 * t86975 + 4.0_f64 / 9.0_f64 * t86979;
    (t87163, t87175)
}
