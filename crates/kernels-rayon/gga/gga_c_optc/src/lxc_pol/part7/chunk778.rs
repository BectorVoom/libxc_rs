//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 778/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk778(t3813: f64, t770: f64, t7451: f64, t2606: f64, t2669: f64, t3625: f64, t2641: f64, t2644: f64, t3814: f64, t896: f64, t9: f64, t769: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7452 = t3813 * t770;
    let t7453 = t7451 * t7452;
    let t7456 = t2669 * t2606;
    let t7457 = t7456 * t3625;
    let t7460 = t2641 * t2606;
    let t7461 = t7460 * t2644;
    let t7464 = t7456 * t3814;
    let t7467 = t9 * t896;
    let t7468 = t7467 * t769;
    (t7452, t7453, t7456, t7457, t7460, t7461, t7464, t7467, t7468)
}
