//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2074/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2074(t2240: f64, t26043: f64, t33: f64, t45844: f64, t6489: f64, t111: f64, t26097: f64, t26351: f64, t6883: f64, t22751: f64, t26186: f64, t26190: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t90312 = t2240 * t33 * t26043;
    let t90330 = t45844 * t6489;
    let t90400 = t26097 * t111;
    let t90459 = t6883 * t26351;
    let t90460 = 0.38381794893125283518e-1_f64 * t90459;
    let t90468 = t22751 * t26186;
    let t90469 = 0.76763589786250567036e-1_f64 * t90468;
    let t90470 = t22751 * t26190;
    (t90312, t90330, t90400, t90460, t90469, t90470)
}
