//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2414/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2414(t17566: f64, t4483: f64, t4475: f64, t60963: f64, t959: f64, t21334: f64, t892: f64, t914: f64, t1580: f64, t49513: f64, t60722: f64, t950: f64) -> (f64, f64, f64, f64) {
    let t68920 = 0.30762056574649219972e4_f64 * t4483 * t17566;
    let t68923 = 0.51947577317044391277e2_f64 * t959 * t60963 * t4475;
    let t68924 = t21334 * t892;
    let t68926 = 1.0_f64 * t68924 * t914;
    let t68930 = 0.30762056574649219973e4_f64 * t49513 * t60722 * t1580 * t950;
    (t68920, t68923, t68926, t68930)
}
