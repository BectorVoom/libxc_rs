//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1460/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1460(t265: f64, t504: f64, t104677: f64, t106606: f64, t109096: f64, t109137: f64, t109743: f64, t109778: f64, t109809: f64, t109844: f64, t109888: f64, t109927: f64, t1256: f64, t1763: f64, t193: f64, t21994: f64, t22408: f64, t24909: f64, t27838: f64, t336: f64, t4700: f64, t6270: f64, t6274: f64, t7398: f64, t86524: f64, t95925: f64) -> f64 {
    let t505 = t265 < t504;
    let t109953 = piecewise3(t505, t193 * t336 * (t109096 + t109137 + t109743 + t109778 + t109809 + t109844 + t109888 + t109927) * t1256 - 3.0_f64 * t4700 * t104677 * t1763 + 6.0_f64 * t4700 * t95925 * t6274 - 3.0_f64 * t4700 * t27838 * t6270 - 6.0_f64 * t4700 * t86524 * t21994 + 6.0_f64 * t4700 * t24909 * t1763 * t6270 - t4700 * t7398 * t22408, t106606);
    t109953
}
