//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 847/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk847(t265: f64, t504: f64, t1256: f64, t1763: f64, t193: f64, t24909: f64, t27838: f64, t28755: f64, t29827: f64, t336: f64, t4700: f64, t6270: f64, t6274: f64, t7398: f64) -> f64 {
    let t505 = t265 < t504;
    let t29840 = piecewise3(t505, t1256 * t193 * t29827 * t336 - 2.0_f64 * t1763 * t27838 * t4700 + 2.0_f64 * t24909 * t4700 * t6274 - t4700 * t6270 * t7398, t28755);
    t29840
}
