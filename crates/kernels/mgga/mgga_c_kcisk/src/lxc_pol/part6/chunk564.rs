//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 564/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk564<F: Float>(t1580: F, t2318: F, t2322: F, t2328: F, t4418: F, t535: F, t541: F, t6456: F, t6459: F, t6474: F, t6498: F, t8308: F, t8319: F, t8324: F, t8328: F, t8332: F, t8337: F, t8400: F) -> (F,) {
    let t8403 = 0.2698618307426597582e-1 * t8308 * t541 + 0.17990788716177317213e-1 * t6456 + 0.17990788716177317213e-1 * t6459 * t2322 - 0.5397236614853195164e-1 * t2318 * t2328 - t4418 + 0.59969295720591057378e-2 * t6474 - 0.17990788716177317213e-1 * t6498 + 0.11993859144118211476e-1 * t1580 * t8319 - 0.17990788716177317213e-1 * t1580 * t8324 - 0.17990788716177317213e-1 * t1580 * t8328 + 0.89953943580886586067e-2 * t1580 * t8332 + 0.5397236614853195164e-1 * t535 * t8337 - 0.2698618307426597582e-1 * t535 * t8400;
    (t8403,)
}
