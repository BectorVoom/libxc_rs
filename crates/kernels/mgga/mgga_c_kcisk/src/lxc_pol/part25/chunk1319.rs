//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1319/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1319<F: Float>(t5030: F, t658: F, t2464: F, t4826: F, t32936: F, t10798: F, t33031: F, t34031: F, t116676: F, t17091: F, t415: F, t717: F, t2537: F, t5176: F, t112512: F, t1799: F, t9945: F) -> (F, F, F, F, F, F) {
    let t117019 = t658 * t5030;
    let t117020 = t4826 * t2464;
    let t117022 = t117019 * t117020 * t32936;
    let t117031 = 0.23148148148148148148e-2 * t33031 * t10798 * t34031;
    let t117033 = 0.23148148148148148148e-2 * t33031 * t116676;
    let t117044 = t415 * t717 * t17091;
    let t117047 = t415 * t5176 * t2537;
    let t117052 = t1799 * t112512 * t9945;
    (t117022, t117031, t117033, t117044, t117047, t117052)
}
