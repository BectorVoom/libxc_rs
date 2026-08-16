//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1335/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1335(t10186: f64, t10241: f64, t10245: f64, t10256: f64, t10328: f64, t2960: f64, t2986: f64, t2988: f64, t41644: f64, t41649: f64, t41705: f64, t41715: f64, t42794: f64, t42799: f64, t42811: f64, t42817: f64, t4510: f64, t4518: f64) -> f64 {
    let t42824 = -0.17777777777777777777e-1_f64 * t10186 * t10256 + 0.22222222222222222222e-2_f64 * t42794 - 0.16666666666666666666e-2_f64 * t2986 * t10241 * t10245 - 0.11111111111111111111e-2_f64 * t2986 * t2988 * t42799 + 0.99999999999999999996e-2_f64 * t2986 * t4518 * t41715 + 0.14814814814814814815e-2_f64 * t2986 * t4510 * t41705 + 0.88888888888888888888e-2_f64 * t2960 * t10328 - 0.32921810699588477364e-2_f64 * t42811 - t42817 - 0.22222222222222222221e-2_f64 * t2986 * t4518 * t41644 - 0.13333333333333333333e-1_f64 * t2986 * t4510 * t41649;
    t42824
}
