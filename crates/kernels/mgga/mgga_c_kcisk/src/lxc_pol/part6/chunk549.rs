//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 549/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk549<F: Float>(t1349: F, t1391: F, t158: F, t173: F, t3844: F, t3848: F, t3851: F, t3852: F, t3858: F, t5802: F, t5804: F, t7710: F, t8123: F, t8126: F, t457: F, t7736: F) -> (F, F) {
    let t8129 = -t3844 - t3848 + t3851 - t3852 + t3858 + 0.11955719325063177623e-1 * t1349 * t7710 - 0.5179538907796306876e-4 * t1391 * t7710 - 0.23911438650126355246e-1 * t5802 + 0.20718155631185227504e-3 * t5804 - 0.10082625e-4 * t173 * t8123 - 0.3513e-2 * t158 * t8126;
    let t8130 = t457 * t7736;
    (t8129, t8130)
}
