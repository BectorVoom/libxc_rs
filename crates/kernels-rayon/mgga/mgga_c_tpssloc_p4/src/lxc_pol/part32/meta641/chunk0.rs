//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2059/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2059(t2240: f64, t3967: f64, t12571: f64, t608: f64, t645: f64, t7445: f64, t26351: f64, t6883: f64, t22751: f64, t26186: f64, t26190: f64, t26356: f64, t6914: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t90104 = t2240 * t3967;
    let t90114 = t12571 * t608;
    let t90247 = t7445 * t645;
    let t90459 = t6883 * t26351;
    let t90460 = 0.38381794893125283518e-1_f64 * t90459;
    let t90468 = t22751 * t26186;
    let t90469 = 0.76763589786250567036e-1_f64 * t90468;
    let t90470 = t22751 * t26190;
    let t90471 = 0.76763589786250567036e-1_f64 * t90470;
    let t90472 = t6914 * t26356;
    (t90104, t90114, t90247, t90460, t90469, t90471, t90472)
}
