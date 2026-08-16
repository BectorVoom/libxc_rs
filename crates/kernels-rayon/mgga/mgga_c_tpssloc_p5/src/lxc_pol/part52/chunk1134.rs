//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1134/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1134(t265: f64, t504: f64, t27421: f64, t27757: f64, t27797: f64, t27832: f64, t3640: f64, t8090: f64, t1254: f64, t1763: f64, t1256: f64, t193: f64, t24905: f64, t24909: f64, t25882: f64, t336: f64, t4700: f64, t5091: f64, t7398: f64) -> f64 {
    let t505 = t265 < t504;
    let t27834 = t27421 + t27757 + t27797 + t27832;
    let t27838 = t8090 * t3640;
    let t27843 = t1763 * t1254;
    let t27850 = piecewise3(t505, t1256 * t193 * t27834 * t336 - t1254 * t27838 * t4700 - t1763 * t24905 * t4700 + 2.0_f64 * t24909 * t27843 * t4700 - t4700 * t5091 * t7398, t25882);
    t27850
}
