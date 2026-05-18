//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1083/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1083<F: Float>(t10481: F, t290: F, t1763: F, t37423: F, t13283: F, t1356: F, t2205: F, t289: F, t45546: F, t45550: F, t45554: F, t45559: F, t45562: F, t45570: F, t45574: F, t45579: F, t45584: F, t45589: F, t45593: F, t45595: F, t45597: F, t45599: F, t5928: F, t9531: F) -> (F, F) {
    let t48530 = t290 * t10481;
    let t48539 = t37423 * t1763;
    let t48545 = -F::new(0.1702583995731913576e-4) * t45546 + F::new(0.5107751987195740728e-4) * t45550 - F::new(0.5107751987195740728e-4) * t45554 - F::new(0.5107751987195740728e-4) * t45559 + F::new(0.5107751987195740728e-4) * t45562 + F::new(0.40911992481368012595e0) * t45570 - F::new(0.8182398496273602519e0) * t45574 - F::new(0.13637330827122670865e0) * t45579 - F::new(0.212822999466489197e-4) * t45584 - F::new(0.2363e1) * t289 * t48530 + F::new(0.79828278012425390428e-1) * t5928 * t9531 + F::new(0.5107751987195740728e-4) * t45589 - F::new(0.59871208509319042821e-1) * t13283 * t2205 - F::new(0.2993560425465952141e-1) * t45593 - F::new(0.11974241701863808564e0) * t1356 * t48539 - F::new(0.5987120850931904282e-1) * t45595 + F::new(0.39726959900411316773e-4) * t45597 - F::new(0.11918087970123395032e-3) * t45599;
    (t48539, t48545)
}
