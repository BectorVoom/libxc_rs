//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1360/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1360(t17210: f64, t5705: f64, t21180: f64, t4362: f64, t17218: f64, t4378: f64, t48103: f64, t68442: f64, t68444: f64, t68446: f64, t68448: f64, t68452: f64, t68454: f64, t68494: f64, t68498: f64, t68500: f64) -> (f64, f64, f64, f64, f64) {
    let t77028 = t17210 * t5705;
    let t77030 = t4362 * t21180;
    let t77032 = t17218 * t5705;
    let t77034 = t4378 * t21180;
    let t77037 = 0.23917333333333333333e1_f64 * t68442 + 0.39862222222222222223e0_f64 * t68444 + 0.44291358024691358024e0_f64 * t68446 - 0.15944888888888888889e1_f64 * t68448 - 0.13145066666666666666e1_f64 * t68452 + 0.21908444444444444444e0_f64 * t68454 + 0.97370864197530864199e0_f64 * t48103 + 0.79724444444444444444e0_f64 * t68494 - 0.23917333333333333333e1_f64 * t68498 + 0.85451625e1_f64 * t77028 - 0.379785e1_f64 * t77030 - 0.46074375e0_f64 * t77032 + 0.614325e0_f64 * t77034 + 0.97370864197530864196e-1_f64 * t68500;
    (t77028, t77030, t77032, t77034, t77037)
}
