//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 686/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk686<F: Float>(t5425: F, t5427: F, t1319: F, t1889: F, t3761: F, t1419: F, t3766: F, t1471: F, t544: F, t1444: F, t1650: F) -> (F, F, F, F, F) {
    let t5428 = t5425 * t5427;
    let t5432 = t3761 * t1889 * t1319;
    let t5436 = t3766 * t1889 * t1419;
    let t5439 = t1471 * t544;
    let t5440 = t1444 * t1650;
    (t5428, t5432, t5436, t5439, t5440)
}
