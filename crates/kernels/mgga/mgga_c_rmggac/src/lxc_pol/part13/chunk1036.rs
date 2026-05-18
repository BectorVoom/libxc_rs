//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1036/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1036<F: Float>(t38460: F, t38426: F, t38428: F, t38432: F, t38436: F, t38442: F, t38448: F, t38450: F, t38457: F, t38465: F, t38467: F, t38469: F, t38473: F, t38477: F, t38483: F, t38485: F, t38487: F) -> F {
    let t42621 = F::new(0.11173207471990682842e-3) * t38460;
    let t42630 = F::new(0.5107751987195740728e-4) * t38426 - F::new(0.5107751987195740728e-4) * t38428 - F::new(0.5107751987195740728e-4) * t38432 + F::new(0.5107751987195740728e-4) * t38436 - F::new(0.212822999466489197e-4) * t38442 - F::new(0.5107751987195740728e-4) * t38448 + F::new(0.5107751987195740728e-4) * t38450 - F::new(0.77813409179935112652e-4) * t38457 - t42621 - F::new(0.638468998399467591e-4) * t38465 - F::new(0.15323255961587222184e-3) * t38467 - F::new(0.5107751987195740728e-4) * t38469 + F::new(0.5107751987195740728e-4) * t38473 - F::new(0.85129199786595678799e-5) * t38477 + F::new(0.2553875993597870364e-4) * t38483 - F::new(0.5107751987195740728e-4) * t38485 + F::new(0.5107751987195740728e-4) * t38487;
    t42630
}
