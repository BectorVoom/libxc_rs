//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2929/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2929(t14667: f64, t17198: f64, t17202: f64, t3209: f64, t4700: f64, t60398: f64, t60400: f64, t60429: f64, t60434: f64, t60568: f64, t60570: f64, t60941: f64, t60946: f64, t60953: f64, t60955: f64, t60958: f64, t60961: f64) -> f64 {
    let t60962 = 8.0_f64 * t14667 * t4700 * t60941 + 2.0_f64 * t17198 * t3209 * t4700 - t17202 * t3209 * t4700 - t60398 + t60400 + t60429 + t60434 + t60568 + t60570 - t60946 - t60953 - t60955 - t60958 + t60961;
    t60962
}
