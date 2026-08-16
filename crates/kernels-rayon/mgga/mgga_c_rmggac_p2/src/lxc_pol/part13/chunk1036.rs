//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1036/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1036(t38460: f64, t38426: f64, t38428: f64, t38432: f64, t38436: f64, t38442: f64, t38448: f64, t38450: f64, t38457: f64, t38465: f64, t38467: f64, t38469: f64, t38473: f64, t38477: f64, t38483: f64, t38485: f64, t38487: f64) -> f64 {
    let t42621 = 0.11173207471990682842e-3_f64 * t38460;
    let t42630 = 0.5107751987195740728e-4_f64 * t38426 - 0.5107751987195740728e-4_f64 * t38428 - 0.5107751987195740728e-4_f64 * t38432 + 0.5107751987195740728e-4_f64 * t38436 - 0.212822999466489197e-4_f64 * t38442 - 0.5107751987195740728e-4_f64 * t38448 + 0.5107751987195740728e-4_f64 * t38450 - 0.77813409179935112652e-4_f64 * t38457 - t42621 - 0.638468998399467591e-4_f64 * t38465 - 0.15323255961587222184e-3_f64 * t38467 - 0.5107751987195740728e-4_f64 * t38469 + 0.5107751987195740728e-4_f64 * t38473 - 0.85129199786595678799e-5_f64 * t38477 + 0.2553875993597870364e-4_f64 * t38483 - 0.5107751987195740728e-4_f64 * t38485 + 0.5107751987195740728e-4_f64 * t38487;
    t42630
}
