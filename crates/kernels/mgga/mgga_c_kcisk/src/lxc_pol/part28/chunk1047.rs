//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1047/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1047<F: Float>(t2364: F, t6707: F, t16580: F, t5182: F, t1636: F, t9019: F, t5192: F, t15862: F, t6981: F, t9035: F, t1333: F, t8862: F, t11197: F, t17740: F, t17751: F, t17757: F, t17766: F, t23886: F, t24049: F, t4823: F) -> (F, F, F, F, F, F, F, F, F) {
    let t24057 = t2364 * t6707;
    let t24058 = t16580 * t24057;
    let t24059 = t5182 * t24058;
    let t24061 = t9019 * t1636;
    let t24062 = t5192 * t24061;
    let t24063 = t5182 * t24062;
    let t24065 = t15862 * t6981;
    let t24066 = t5182 * t24065;
    let t24068 = t9035 * t1636;
    let t24069 = t5192 * t24068;
    let t24070 = t5182 * t24069;
    let t24073 = t1333 * t8862;
    let t24075 = 0.74498e-1 * t4823 * t24049 - 0.43134342e-1 * t11197 * t23886 - 0.7369753086419753086e-3 * t24059 + t17740 - 0.36848765432098765431e-3 * t24063 - 0.58958024691358024689e-2 * t24066 + 0.11054629629629629629e-2 * t24070 - t17751 - 0.44218518518518518516e-2 * t17757 + t17766 - 0.88437037037037037033e-2 * t24073;
    (t24057, t24059, t24061, t24063, t24066, t24068, t24070, t24073, t24075)
}
