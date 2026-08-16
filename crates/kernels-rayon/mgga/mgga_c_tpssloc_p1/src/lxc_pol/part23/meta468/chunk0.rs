//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1374/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1374(t2792: f64, t76998: f64, t913: f64, t10632: f64, t41825: f64, t76637: f64, t959: f64, t5742: f64, t48103: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t68494: f64, t68498: f64, t68500: f64, t77028: f64, t77030: f64, t77032: f64, t77034: f64) -> (f64, f64, f64, f64) {
    let t77232 = 6.0_f64 * t2792 * t76998 * t913;
    let t77236 = 0.12304822629859687989e5_f64 * t959 * t41825 * t76637 * t10632;
    let t77239 = t5742 * t5742;
    let t77257 = 0.41318e1_f64 * t68442 + 0.68863333333333333332e0_f64 * t68444 + 0.76514814814814814814e0_f64 * t68446 - 0.27545333333333333332e1_f64 * t68448 - 0.166712e1_f64 * t68452 + 0.27785333333333333333e0_f64 * t68454 + 0.12349037037037037037e1_f64 * t48103 + 0.13772666666666666667e1_f64 * t68494 - 0.41318e1_f64 * t68498 + 0.158837625e2_f64 * t77028 - 0.705945e1_f64 * t77030 - 0.94674375e0_f64 * t77032 + 0.1262325e1_f64 * t77034 + 0.12349037037037037037e0_f64 * t68500;
    (t77232, t77236, t77239, t77257)
}
