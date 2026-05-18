//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 379/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk379<F: Float>(t1409: F, t1419: F, t1401: F, t317: F, t334: F, t522: F, t41: F, t451: F) -> (F, F, F, F) {
    let t1420 = t1409 * t1419;
    let t1424 = F::new(0.11955719325063177623e-1) * t1401;
    let t1429 = F::new(0.3513e-2) * t317 * t334 * t522;
    let t1430 = t41 * t451;
    (t1420, t1424, t1429, t1430)
}
