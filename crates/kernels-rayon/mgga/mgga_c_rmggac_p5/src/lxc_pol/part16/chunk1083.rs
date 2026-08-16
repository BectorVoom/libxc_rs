//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1083/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1083(t10481: f64, t290: f64, t1763: f64, t37423: f64, t13283: f64, t1356: f64, t2205: f64, t289: f64, t45546: f64, t45550: f64, t45554: f64, t45559: f64, t45562: f64, t45570: f64, t45574: f64, t45579: f64, t45584: f64, t45589: f64, t45593: f64, t45595: f64, t45597: f64, t45599: f64, t5928: f64, t9531: f64) -> (f64, f64) {
    let t48530 = t290 * t10481;
    let t48539 = t37423 * t1763;
    let t48545 = -0.1702583995731913576e-4_f64 * t45546 + 0.5107751987195740728e-4_f64 * t45550 - 0.5107751987195740728e-4_f64 * t45554 - 0.5107751987195740728e-4_f64 * t45559 + 0.5107751987195740728e-4_f64 * t45562 + 0.40911992481368012595e0_f64 * t45570 - 0.8182398496273602519e0_f64 * t45574 - 0.13637330827122670865e0_f64 * t45579 - 0.212822999466489197e-4_f64 * t45584 - 0.2363e1_f64 * t289 * t48530 + 0.79828278012425390428e-1_f64 * t5928 * t9531 + 0.5107751987195740728e-4_f64 * t45589 - 0.59871208509319042821e-1_f64 * t13283 * t2205 - 0.2993560425465952141e-1_f64 * t45593 - 0.11974241701863808564e0_f64 * t1356 * t48539 - 0.5987120850931904282e-1_f64 * t45595 + 0.39726959900411316773e-4_f64 * t45597 - 0.11918087970123395032e-3_f64 * t45599;
    (t48539, t48545)
}
