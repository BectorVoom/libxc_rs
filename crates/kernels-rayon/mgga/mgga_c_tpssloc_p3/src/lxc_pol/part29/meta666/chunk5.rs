//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2224/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2224(t26421: f64, t26446: f64, t3734: f64, t90591: f64, t1336: f64, t22710: f64, t22874: f64, t22877: f64, t26403: f64, t26456: f64, t26458: f64, t3777: f64, t3793: f64, t3851: f64, t3856: f64, t5234: f64, t5250: f64, t5334: f64, t5344: f64, t81160: f64, t81184: f64, t81189: f64, t90946: f64, t91025: f64, t91029: f64, t91036: f64, t91043: f64, t91045: f64, t91048: f64) -> f64 {
    let t91052 = t90591 * t26446 * t26421 * t3734;
    let t91059 = -0.76763589786250567036e-1_f64 * t81160 - 0.38381794893125283518e-1_f64 * t81184 - 0.16449340668482264365e-1_f64 * t91025 + 2.0_f64 * t5234 * t22710 + 2.0_f64 * t1336 * t91029 * t3793 - 2.0_f64 * t3777 * t26456 + 0.3289868133696452873e-1_f64 * t91036 + 4.0_f64 * t5334 * t90946 * t5250 - t1336 * t26458 * t3856 - t91043 + 0.76763589786250567036e-1_f64 * t81189 + t91045 + 0.9869604401089358619e-1_f64 * t91048 - 0.19739208802178717238e0_f64 * t91052 - t5344 * t26403 * t3851 - 2.0_f64 * t5234 * t22874 - t5234 * t22877;
    t91059
}
