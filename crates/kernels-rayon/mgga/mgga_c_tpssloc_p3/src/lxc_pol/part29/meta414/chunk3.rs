//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1680/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1680(t12236: f64, t1315: f64, t16083: f64, t16086: f64, t16090: f64, t16099: f64, t16101: f64, t16103: f64, t16106: f64, t16108: f64, t16113: f64, t16115: f64, t16119: f64, t5195: f64) -> f64 {
    let t16121 = -t16083 + 0.99999999999999999996e-2_f64 * t5195 * t16086 + 0.49999999999999999998e-2_f64 * t5195 * t16090 - t16099 - t12236 - 0.19999999999999999999e-1_f64 * t16101 * t16103 + 0.77777777777777777774e-2_f64 * t16106 - 0.52777777777777777776e-2_f64 * t16108 + t16113 - 0.16666666666666666666e-2_f64 * t1315 * t16115 + 0.16666666666666666666e-2_f64 * t16119;
    t16121
}
