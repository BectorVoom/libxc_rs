//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1142/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1142(t148475: f64, t446: f64, t9049: f64, t1369: f64, t147944: f64, t2112: f64, t28: f64, t1039: f64, t32869: f64, t586: f64, t5890: f64, t139361: f64, t148435: f64, t148439: f64, t148443: f64, t148446: f64, t148449: f64, t148454: f64, t148457: f64, t148460: f64, t148464: f64, t148467: f64, t148470: f64, t148473: f64) -> (f64, f64, f64, f64) {
    let t148477 = t446 * t9049 * t148475;
    let t148481 = t1369 * t28 * t2112 * t147944;
    let t148486 = t5890 * t28 * t586 * t32869 * t1039;
    let t148488 = -t148435 / 3.0_f64 - 2.0_f64 * t148439 + t148443 - 2.0_f64 / 3.0_f64 * t148446 - 2.0_f64 / 9.0_f64 * t148449 - 2.0_f64 / 3.0_f64 * t148454 - 8.0_f64 / 9.0_f64 * t148457 + t148460 / 18.0_f64 - 8.0_f64 / 9.0_f64 * t139361 - 8.0_f64 / 9.0_f64 * t148464 + 2.0_f64 / 3.0_f64 * t148467 - 4.0_f64 / 9.0_f64 * t148470 - 2.0_f64 / 9.0_f64 * t148473 + 2.0_f64 / 27.0_f64 * t148477 + 2.0_f64 / 3.0_f64 * t148481 + t148486 / 12.0_f64;
    (t148477, t148481, t148486, t148488)
}
