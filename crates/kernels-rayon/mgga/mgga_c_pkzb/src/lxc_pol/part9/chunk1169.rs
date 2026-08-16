//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1169/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1169(t501: f64, t7028: f64, t496: f64, t4874: f64, t7046: f64, t4877: f64, t2609: f64, t5331: f64, t1667: f64, t6801: f64, t5336: f64, t16935: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20362 = t501 * t7028;
    let t20363 = 24.0_f64 * t20362;
    let t20365 = 24.0_f64 * t496 * t7028;
    let t20366 = t7046 * t4874;
    let t20367 = 0.21687162600603479684e-1_f64 * t20366;
    let t20368 = t7046 * t4877;
    let t20369 = 0.32530743900905219526e-1_f64 * t20368;
    let t20370 = t2609 * t5331;
    let t20371 = 0.35089341735807877242e1_f64 * t20370;
    let t20372 = t6801 * t1667;
    let t20373 = 0.73245789224026180216e-3_f64 * t20372;
    let t20374 = t2609 * t5336;
    let t20375 = 0.51947577317044391277e2_f64 * t20374;
    let t20376 = 0.17090684152272775383e-2_f64 * t16935;
    (t20363, t20365, t20367, t20369, t20371, t20373, t20375, t20376)
}
