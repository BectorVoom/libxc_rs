//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2297/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2297(t3597: f64, t6243: f64, t6238: f64, t7299: f64, t1090: f64, t18241: f64, t19120: f64, t19214: f64, t19226: f64, t2121: f64, t2155: f64, t225: f64, t24589: f64, t24601: f64, t24880: f64, t27403: f64, t27406: f64, t27438: f64, t29678: f64, t29798: f64, t3487: f64, t462: f64, t497: f64, t6244: f64, t66845: f64, t7283: f64, t7285: f64, t7286: f64, t7296: f64, t7302: f64, t7351: f64, t94395: f64, t94628: f64, t94631: f64) -> (f64, f64) {
    let t103345 = t3597 * t6243;
    let t103363 = t7299 * t6238;
    let t103377 = -6.0_f64 * t7351 * t19226 - 0.54831135561607547883e-2_f64 * t24589 * t24601 * t103345 * t1090 - 0.14621636149762012769e-1_f64 * t94395 * t27438 - 0.27415567780803773942e-2_f64 * t7283 * t7285 * t7286 * t18241 - 6.0_f64 * t3487 * t29798 + 0.43864908449286038306e-1_f64 * t27406 * t27403 + t94628 + 4.0_f64 * t7351 * t19214 + 0.48738787165873375897e-2_f64 * t94631 - 0.82246703342411321825e-2_f64 * t7283 * t103363 * t7302 + 0.82246703342411321825e-2_f64 * t2121 * t462 * t19120 * t225 * t497 - t66845 * t2155 + 2.0_f64 * t24880 * t6244 + 0.80418998823691070228e-1_f64 * t29678 * t7296;
    (t103345, t103377)
}
