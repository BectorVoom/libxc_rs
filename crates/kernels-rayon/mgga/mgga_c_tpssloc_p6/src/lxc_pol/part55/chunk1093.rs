//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1093/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1093(t214: f64, t32748: f64, t1985: f64, t1825: f64, t31211: f64, t32726: f64, t553: f64, t1336: f64, t1814: f64, t31192: f64, t31200: f64, t32743: f64, t32747: f64, t544: f64, t8483: f64) -> (f64, f64, f64, f64) {
    let t32749 = t214 * t32748;
    let t32751 = 0.16449340668482264365e-1_f64 * t1985 * t32749;
    let t32753 = t31211 * t1825;
    let t32755 = t553 * t32726;
    let t32757 = -t1336 * t32753 + t1814 * t8483 + t32755 * t544 - t31192 - t31200 - t32743 - t32747 + t32751;
    (t32749, t32753, t32755, t32757)
}
