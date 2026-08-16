//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 760/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk760(t7: f64, t143: f64, t1270: f64, t1285: f64, t172: f64, t187: f64, t4045: f64, t4046: f64, t4082: f64, t139: f64, t214: f64, t26: f64, t3804: f64, t2170: f64, t3814: f64, t776: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64) {
    let t8 = t7 <= zeta_threshold;
    let t144 = 0.135e1_f64 <= t143;
    let t4086 = piecewise3(t144, t4045, -8.0_f64 / 3.0_f64 * t4046 * t187 - 16.0_f64 / 3.0_f64 * t1270 * t1285 - 8.0_f64 / 3.0_f64 * t172 * t4082);
    let t4087 = t139 * t4086;
    let t4088 = t4087 * t214;
    let t4089 = t26 * t4088;
    let t4094 = piecewise3(t8, 0.0_f64, t3804);
    let t4104 = piecewise3(t8, 0.0_f64, 4.0_f64 / 9.0_f64 * t2170 * t3814 - t776 * t3804 / 3.0_f64);
    (t4086, t4088, t4089, t4094, t4104)
}
