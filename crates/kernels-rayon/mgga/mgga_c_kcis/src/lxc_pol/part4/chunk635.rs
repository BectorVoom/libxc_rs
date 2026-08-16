//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 635/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk635(t3451: f64, t387: f64, t1187: f64, t1184: f64, t1196: f64, t1200: f64, t3316: f64, t359: f64, t376: f64, t1170: f64, t3225: f64, t373: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3452 = t387 * t3451;
    let t3453 = t1187 * t3452;
    let t3455 = t1184 * t1196;
    let t3457 = t1184 * t1200;
    let t3459 = t359 * t3316;
    let t3460 = t376 * t3459;
    let t3461 = t1170 * t3460;
    let t3463 = t373 * t3225;
    (t3452, t3453, t3455, t3457, t3460, t3461, t3463)
}
