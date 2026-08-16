//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 778/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk778(t3236: f64, t1124: f64, t1128: f64, t1127: f64, t432: f64) -> (f64, f64, f64, f64) {
    let t3319 = 0.22831111111111111111e-1_f64 * t3236;
    let t3327 = t1124 * t1128;
    let t3330 = t1127 * t432;
    let t3331 = 1.0_f64 / t3330;
    (t3319, t3327, t3330, t3331)
}
