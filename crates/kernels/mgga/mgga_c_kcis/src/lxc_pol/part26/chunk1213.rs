//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1213/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1213<F: Float>(t102115: F, t102118: F, t102121: F, t102124: F, t102127: F, t102129: F, t102155: F, t103114: F, t103119: F, t12231: F, t1928: F, t27369: F, t28353: F, t98308: F, t98344: F, t990: F) -> (F,) {
    let t103366 = 0.99024918276041666664e-4 * t12231 * t1928 * t990 * t28353 - 0.2653111111111111111e-1 * t102115 + 0.88437037037037037033e-2 * t102118 - 0.88437037037037037034e-2 * t102121 + 0.17687407407407407407e-1 * t102124 - 0.33163888888888888888e-2 * t102127 + 0.10297067901234567901e-3 * t98308 + 0.1621345679012345679e-1 * t102129 - 0.44218518518518518516e-2 * t98344 + 0.61836467013888888889e-4 * t27369 * t103114 - 0.12367293402777777778e-3 * t27369 * t103119 - 0.44218518518518518516e-2 * t102155;
    (t103366,)
}
