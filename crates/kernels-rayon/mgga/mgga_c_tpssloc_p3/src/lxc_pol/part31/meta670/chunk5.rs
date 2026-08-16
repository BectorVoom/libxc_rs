//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1994/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1994(t100651: f64, t100682: f64, t100692: f64, t100713: f64, t100743: f64, t101196: f64, t101211: f64, t101220: f64, t101241: f64, t1649: f64, t1877: f64, t2057: f64, t24191: f64, t2522: f64, t25892: f64, t25905: f64, t25921: f64, t26563: f64, t26740: f64, t26756: f64, t28774: f64, t28778: f64, t7110: f64, t7649: f64, t7845: f64, t92319: f64) -> f64 {
    let t102012 = 3.0_f64 / 2.0_f64 * t2522 * t2057 * t100743 - 3.0_f64 * t26756 * t100682 + 6.0_f64 * t101196 * t25892 + t1877 * t26740 * t1649 + 3.0_f64 / 2.0_f64 * t2522 * t2057 * t100692 + 6.0_f64 * t24191 * t100713 + t101211 - t101220 - 3.0_f64 * t92319 * t25921 + 3.0_f64 * t2522 * t7110 * t28774 + 3.0_f64 / 2.0_f64 * t2522 * t7110 * t28778 + 3.0_f64 * t2522 * t7845 * t25905 + 3.0_f64 * t2522 * t26740 * t7649 - t101241 + 6.0_f64 * t26563 * t100651;
    t102012
}
