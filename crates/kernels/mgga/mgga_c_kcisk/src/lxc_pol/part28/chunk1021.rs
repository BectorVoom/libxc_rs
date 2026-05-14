//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1021/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1021<F: Float>(t1248: F, t4889: F, t8514: F, t11036: F, t8701: F, t1709: F, t1720: F, t22501: F, t22387: F, t4893: F, t11003: F, t22392: F, t17480: F, t22396: F, t22484: F, t22488: F, t7130: F) -> (F, F, F, F, F, F, F, F) {
    let t23570 = t1248 * t4889 * t8514;
    let t23575 = t11036 * t8701;
    let t23576 = t23575 * t1709;
    let t23579 = t1248 * t1720 * t22501;
    let t23583 = t1248 * t4893 * t22387;
    let t23587 = t1248 * t11003 * t22392;
    let t23590 = t1248 * t17480 * t22396;
    let t23593 = t1248 * t1720 * t22484;
    let t23596 = t1248 * t7130 * t22488;
    (t23570, t23576, t23579, t23583, t23587, t23590, t23593, t23596)
}
