//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 816/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk816<F: Float>(t1409: F, t7141: F, t4023: F, t4047: F, t4050: F, t4053: F, t4054: F, t4060: F, t5814: F, t5816: F, t5829: F, t5831: F, t5833: F, t6281: F, t1451: F, t6912: F) -> (F, F, F) {
    let t7142 = t1409 * t7141;
    let t7155 = t4047 - t4050 - t4053 - 0.23911438650126355246e-1 * t5814 + 0.20718155631185227504e-3 * t5816 - t4054 + t4060 - 0.23526125e-4 * t5829 + 0.9368e-2 * t5831 - 0.26416666666666666666e-2 * t5833 - 0.23911438650126355246e-1 * t4023 * t6281;
    let t7158 = t1451 * t6912;
    (t7142, t7155, t7158)
}
