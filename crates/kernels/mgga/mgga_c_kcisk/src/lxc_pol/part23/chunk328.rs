//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 328/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk328<F: Float>(t1434: F, t457: F, t1216: F, t1419: F, t1421: F, t1425: F, t1430: F, t338: F, t456: F) -> (F, F) {
    let t1435 = t457 * t1434;
    let t1440 = t1419 + 0.65704296666666666667e-3 * t1421 * t1425 + 0.1478346675e-2 * t456 * t1430 - 0.98556445e-3 * t456 * t1435 - 4.0 * t338 * t1216;
    (t1435, t1440)
}
