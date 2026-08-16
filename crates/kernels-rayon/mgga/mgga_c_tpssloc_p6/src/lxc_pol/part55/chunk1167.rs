//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1167/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1167(t24574: f64, t32511: f64, t32544: f64, t85660: f64, t8872: f64, t225: f64, t32452: f64, t32422: f64, t24826: f64, t32466: f64, t1089: f64, t2144: f64, t7327: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t118052 = t24574 * t32511;
    let t118059 = t24574 * t32544;
    let t118067 = 0.36554090374405031922e-2_f64 * t85660 * t8872;
    let t118084 = t32452 * t225;
    let t118097 = t32422 * t225;
    let t118111 = t24826 * t32466;
    let t118136 = t7327 * t2144 * t1089;
    (t118052, t118059, t118067, t118084, t118097, t118111, t118136)
}
