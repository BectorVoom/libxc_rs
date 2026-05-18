//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1219/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1219<F: Float>(t871: F, t8710: F, t27837: F, t27840: F, t27844: F, t27856: F, t27858: F, t27860: F, t32300: F, t32302: F, t32304: F, t739: F) -> (F, F) {
    let t32305 = t8710 * t871;
    let t32307 = F::new(63.0) / F::new(512.0) * t27837;
    let t32308 = F::new(385.0) / F::new(16384.0) * t27840;
    let t32309 = F::new(147.0) / F::new(1048576.0) * t27844;
    let t32310 = F::new(49.0) / F::new(1048576.0) * t27856;
    let t32311 = F::new(385.0) / F::new(49152.0) * t27858;
    let t32312 = F::new(21.0) / F::new(512.0) * t27860;
    let t32313 = t32300 - t32302 + t32304 + t32305 / F::new(2.0) + t32307 - t32308 + t32309 - t32310 + t32311 - t32312;
    let t32314 = t739 * t32313;
    (t32313, t32314)
}
