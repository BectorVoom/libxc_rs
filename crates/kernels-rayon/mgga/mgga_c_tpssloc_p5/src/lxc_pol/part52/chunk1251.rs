//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1251/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1251(t22690: f64, t23171: f64, t30676: f64, t30725: f64, t814: f64, t23012: f64, t8332: f64, t8336: f64, t225: f64, t30732: f64, t40772: f64, t8369: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t113005 = 0.16449340668482264365e-1_f64 * t23171 * t22690 * t30676;
    let t113016 = t814 * t30725;
    let t113038 = 0.12793931631041761173e0_f64 * t23012 * t8332;
    let t113045 = 0.12793931631041761173e0_f64 * t23012 * t8336;
    let t113053 = t30732 * t225;
    let t113082 = t8369 * t40772;
    (t113005, t113016, t113038, t113045, t113053, t113082)
}
