//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1074/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1074<F: Float>(t10657: F, t64: F, t3427: F, t90: F, t27837: F, t27840: F, t27844: F, t27856: F, t27858: F, t27860: F, t10691: F, t21665: F, t2932: F, t7064: F, t7177: F, t10698: F, t1841: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t32302 = 8.0 / 3.0 * t10657 * t64;
    let t32304 = 4.0 / 3.0 * t3427 * t90;
    let t32307 = 63.0 / 512.0 * t27837;
    let t32308 = 385.0 / 16384.0 * t27840;
    let t32309 = 147.0 / 1048576.0 * t27844;
    let t32310 = 49.0 / 1048576.0 * t27856;
    let t32311 = 385.0 / 49152.0 * t27858;
    let t32312 = 21.0 / 512.0 * t27860;
    let t32328 = t21665 * t10691;
    let t32329 = 0.64087718584518535698e-3 * t32328;
    let t32331 = t7064 * t2932 * t7177;
    let t32332 = 0.32043859292259267849e-3 * t32331;
    let t32333 = t1841 * t10698;
    (t32302, t32304, t32307, t32308, t32309, t32310, t32311, t32312, t32329, t32332, t32333)
}
