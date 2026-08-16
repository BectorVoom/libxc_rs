//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2938/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2938(t52035: f64, t52037: f64, t63338: f64, t63340: f64, t63342: f64, t63361: f64, t63371: f64, t77539: f64, t77543: f64, t77547: f64, t77799: f64, t52128: f64, t52751: f64, t63447: f64, t63453: f64, t63459: f64, t77802: f64, t77804: f64, t77806: f64, t77810: f64, t77813: f64, t77816: f64, t77819: f64) -> (f64, f64) {
    let t78049 = -0.53814e1_f64 * t77539 + 0.17938e1_f64 * t77543 + 0.17938e1_f64 * t77547 - 0.11958666666666666667e1_f64 * t63338 + 0.39862222222222222222e0_f64 * t63340 + 0.33218518518518518518e0_f64 * t63342 + 0.17938e1_f64 * t63361 - 0.11958666666666666667e1_f64 * t63371 + 0.79724444444444444446e0_f64 * t52035 - 0.26574814814814814815e0_f64 * t52037 + 0.3071625e0_f64 * t77799;
    let t78061 = 0.1898925e1_f64 * t77802 - 0.32862666666666666666e0_f64 * t77804 + 0.54771111111111111112e-1_f64 * t77806 - t52751 + 0.73028148148148148149e0_f64 * t52128 + 0.197176e1_f64 * t77810 - 0.147882e1_f64 * t77813 + 0.49293999999999999999e0_f64 * t77816 + 0.49293999999999999999e0_f64 * t77819 + 0.29896666666666666667e0_f64 * t63447 - 0.26574814814814814815e0_f64 * t63453 + 0.79724444444444444444e0_f64 * t63459;
    (t78049, t78061)
}
