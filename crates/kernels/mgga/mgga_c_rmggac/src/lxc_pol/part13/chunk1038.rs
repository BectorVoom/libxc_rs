//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1038/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1038<F: Float>(t38559: F, t38562: F, t38539: F, t38541: F, t38545: F, t38550: F, t38552: F, t38554: F, t38556: F, t38566: F, t38570: F, t38572: F, t38574: F, t38576: F, t38578: F, t38583: F, t38588: F) -> F {
    let t42665 = F::new(0.162600798888400151e-2) * t38559;
    let t42666 = F::new(0.162600798888400151e-2) * t38562;
    let t42675 = -F::new(0.638468998399467591e-4) * t38539 + F::new(0.1702583995731913576e-4) * t38541 + F::new(0.1702583995731913576e-4) * t38545 + F::new(0.638468998399467591e-4) * t38550 + F::new(0.60975299583150056624e-3) * t38552 + F::new(0.60975299583150056624e-3) * t38554 - F::new(0.7044137609176975208e-2) * t38556 - t42665 - t42666 + F::new(0.40911992481368012596e0) * t38566 - F::new(0.14546486215597515589e0) * t38570 + F::new(0.10215503974391481456e-3) * t38572 - F::new(0.15323255961587222184e-3) * t38574 - F::new(0.5107751987195740728e-4) * t38576 + F::new(0.5107751987195740728e-4) * t38578 - F::new(0.638468998399467591e-4) * t38583 + F::new(0.3405167991463827152e-4) * t38588;
    t42675
}
