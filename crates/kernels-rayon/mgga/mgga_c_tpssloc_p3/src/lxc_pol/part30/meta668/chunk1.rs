//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2096/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2096(t91344: f64, t26245: f64, t80783: f64, t22897: f64, t6925: f64, t26302: f64, t80958: f64, t22779: f64, t26323: f64, t1336: f64, t242: f64, t80901: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t91345 = 0.28260929265898273598e-2_f64 * t91344;
    let t91346 = t80783 * t26245;
    let t91351 = t6925 * t22897;
    let t91356 = t80958 * t26302;
    let t91357 = 0.16956557559538964159e-1_f64 * t91356;
    let t91358 = t22779 * t26323;
    let t91359 = 0.28260929265898273598e-2_f64 * t91358;
    let t91361 = t1336 * t80901 * t242;
    (t91345, t91346, t91351, t91357, t91359, t91361)
}
