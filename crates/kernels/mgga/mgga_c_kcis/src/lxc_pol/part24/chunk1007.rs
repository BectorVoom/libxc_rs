//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1007/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1007<F: Float>(t2205: F, t6860: F, t29045: F, t29047: F, t29049: F, t29052: F, t29054: F, t29057: F, t29060: F, t29063: F, t29065: F, t29067: F, t29069: F, t29071: F, t29073: F, t29075: F, t29077: F, t29079: F) -> (F, F) {
    let t29188 = t2205 * t6860;
    let t29214 = 0.9375e-1 * t29045 - 0.1875e0 * t29047 + 0.125e0 * t29049 + 0.1875e0 * t29052 - 0.125e0 * t29054 - 0.9375e-1 * t29057 - 0.20833333333333333333e-1 * t29060 + 0.625e-1 * t29063 - 0.20234375e-1 * t29065 + 0.4046875e-1 * t29067 - 0.53958333333333333334e-1 * t29069 - 0.4046875e-1 * t29071 + 0.53958333333333333334e-1 * t29073 + 0.20234375e-1 * t29075 - 0.89930555555555555557e-2 * t29077 - 0.26979166666666666667e-1 * t29079;
    (t29188, t29214)
}
