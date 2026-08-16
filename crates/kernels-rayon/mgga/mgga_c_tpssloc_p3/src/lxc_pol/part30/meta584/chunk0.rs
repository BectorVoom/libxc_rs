//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1963/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1963(t1351: f64, t6387: f64, t6330: f64, t12250: f64, t1834: f64, t5286: f64, t1824: f64, t5318: f64, t1372: f64, t6414: f64, t19731: f64, t562: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t57091 = t6387 * t1351;
    let t57172 = t6330 * t1351;
    let t57342 = t6387 * t12250;
    let t57499 = t1834 * t5286;
    let t57545 = t5318 * t1824;
    let t57607 = t1372 * t6387;
    let t57618 = t1372 * t6414;
    let t57704 = t562 * t19731;
    (t57091, t57172, t57342, t57499, t57545, t57607, t57618, t57704)
}
