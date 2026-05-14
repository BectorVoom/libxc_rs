//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1043/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1043<F: Float>(t11708: F, t8240: F, t12538: F, t6395: F, t10868: F, t2147: F, t9292: F, t10760: F, t30637: F, t6085: F, t30643: F, t11678: F, t7601: F, t30140: F, t30856: F, t6093: F) -> (F, F, F, F, F, F, F, F) {
    let t43516 = t8240 * t11708;
    let t43518 = t6395 * t12538;
    let t43521 = t2147 * t10868 * t9292;
    let t43528 = t6085 * t10760 * t30637;
    let t43531 = t6085 * t10760 * t30643;
    let t43533 = t7601 * t11678;
    let t43536 = t6085 * t10760 * t30140;
    let t43539 = t6093 * t10760 * t30856;
    (t43516, t43518, t43521, t43528, t43531, t43533, t43536, t43539)
}
