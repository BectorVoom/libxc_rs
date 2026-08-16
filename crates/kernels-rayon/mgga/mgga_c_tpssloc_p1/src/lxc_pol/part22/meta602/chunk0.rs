//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2124/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2124(t10701: f64, t1543: f64, t10810: f64, t1561: f64, t47705: f64, t47707: f64, t48096: f64, t47730: f64, t48155: f64, t48157: f64, t2929: f64, t4446: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t49274 = t1543 * t10701;
    let t49285 = t1561 * t10810;
    let t49304 = 0.13772666666666666666e1_f64 * t47705;
    let t49306 = 0.45908888888888888888e0_f64 * t47707;
    let t49317 = 0.34731666666666666667e0_f64 * t48096;
    let t49322 = 0.68863333333333333332e0_f64 * t47730;
    let t49378 = 0.69463333333333333334e0_f64 * t48155;
    let t49379 = 0.11577222222222222222e0_f64 * t48157;
    let t49411 = t4446 * t2929;
    (t49274, t49285, t49304, t49306, t49317, t49322, t49378, t49379, t49411)
}
