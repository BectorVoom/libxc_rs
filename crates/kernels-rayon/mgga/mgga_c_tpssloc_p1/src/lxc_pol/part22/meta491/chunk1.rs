//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1916/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1916(t11094: f64, t1637: f64, t17202: f64, t193: f64, t21093: f64, t21097: f64, t21099: f64, t21103: f64, t21105: f64, t21107: f64, t21365: f64, t21367: f64, t21369: f64, t21372: f64, t21375: f64, t21376: f64, t336: f64, t4700: f64) -> f64 {
    let t21381 = 2.0_f64 * t11094 * t193 * t21376 * t336 - 3.0_f64 * t1637 * t17202 * t4700 - t21093 + t21097 - t21099 - t21103 - t21105 - t21107 + t21365 + t21367 + t21369 - t21372 + t21375;
    t21381
}
