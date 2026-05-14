//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1079/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1079<F: Float>(t32049: F, t10172: F, t30182: F, t30184: F, t30186: F, t32021: F, t32025: F, t32028: F, t32036: F, t32038: F, t32041: F, t32043: F, t32045: F, t32047: F, t4141: F, t10167: F, t29874: F) -> (F, F) {
    let t32050 = 0.23712505529730124666e-2 * t32049;
    let t32051 = -t30182 + t32021 - t32025 + t32028 + 0.31616674039640166222e-2 * t4141 * t10172 + t32036 + t32038 - t32041 + t32043 + t32045 + t32047 - t30184 + t30186 + t32050;
    let t32052 = t29874 * t10167;
    (t32051, t32052)
}
