//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1448/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1448(t11195: f64, t14720: f64, t14766: f64, t14886: f64, t14890: f64, t18203: f64, t18208: f64, t18213: f64, t18217: f64, t18219: f64, t18223: f64, t18229: f64, t18234: f64, t18243: f64, t18494: f64, t18505: f64, t18512: f64, t18521: f64, t18731: f64, t18762: f64, t18810: f64, t18832: f64) -> f64 {
    let t18834 = 0.1898925e1_f64 * t18731 - t11195 - 0.54771111111111111111e-1_f64 * t18512 + 0.82156666666666666667e-1_f64 * t18521 + 0.66437037037037037037e-1_f64 * t18203 - 0.19931111111111111111e0_f64 * t18219 - 0.99655555555555555557e-1_f64 * t18229 + 0.29896666666666666667e0_f64 * t18243 + 0.18257037037037037037e-1_f64 * t18494 - 0.10954222222222222222e0_f64 * t18505 + t18810 - 0.9494625e0_f64 * t18762 + 0.18257037037037037037e0_f64 * t14766 + 0.13287407407407407407e0_f64 * t14720 - t14886 - t14890 - 0.19931111111111111111e0_f64 * t18234 + 0.33218518518518518518e0_f64 * t18208 - 0.11958666666666666667e1_f64 * t18213 - 0.39862222222222222222e0_f64 * t18217 + 0.17938e1_f64 * t18223 + t18832;
    t18834
}
