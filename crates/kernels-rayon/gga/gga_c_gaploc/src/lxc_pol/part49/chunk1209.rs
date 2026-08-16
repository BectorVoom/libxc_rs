//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1209/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1209(t13813: f64, t1562: f64, t4614: f64, t12078: f64, t1415: f64, t7030: f64, t47953: f64, t6716: f64, t6717: f64, t42422: f64, t42425: f64, t42429: f64, t42432: f64, t42435: f64, t42438: f64, t42442: f64, t42444: f64, t42448: f64) -> f64 {
    let t48205 = t1562 * t4614 * t13813;
    let t48208 = t1415 * t12078 * t7030;
    let t48211 = t6716 * t6717 * t47953;
    let t48215 = -0.19171462976960374838e0_f64 * t42422 - 0.19171462976960374838e0_f64 * t42425 + t42429 - t42432 - 0.92023022289409799224e1_f64 * t48205 - 0.14896037479937677779e-1_f64 * t48208 + 0.69017266717057349418e1_f64 * t48211 - 0.79445533226334281487e-1_f64 * t42435 + t42438 + t42442 - t42444 - 0.7150097990370085334e0_f64 * t42448;
    t48215
}
