//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1204/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1204<F: Float>(t10657: F, t64: F, t3427: F, t90: F, t27837: F, t27840: F, t27844: F, t27856: F, t27858: F, t27860: F, t10691: F, t21665: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32302 = F::new(8.0) / F::new(3.0) * t10657 * t64;
    let t32304 = F::new(4.0) / F::new(3.0) * t3427 * t90;
    let t32307 = F::new(63.0) / F::new(512.0) * t27837;
    let t32308 = F::new(385.0) / F::new(16384.0) * t27840;
    let t32309 = F::new(147.0) / F::new(1048576.0) * t27844;
    let t32310 = F::new(49.0) / F::new(1048576.0) * t27856;
    let t32311 = F::new(385.0) / F::new(49152.0) * t27858;
    let t32312 = F::new(21.0) / F::new(512.0) * t27860;
    let t32328 = t21665 * t10691;
    (t32302, t32304, t32307, t32308, t32309, t32310, t32311, t32312, t32328)
}
