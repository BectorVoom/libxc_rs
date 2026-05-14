//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 744/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk744<F: Float>(t4982: F, t89: F, t4721: F, t4791: F, t4794: F, t4961: F, t4964: F, t4967: F, t4969: F, t4972: F, t4975: F, t4977: F, t4979: F, t4981: F, t1385: F, t732: F) -> (F, F, F, F) {
    let t4983 = t4982 * t89;
    let t4984 = 144.0 * t4983;
    let t4985 = t4961 + t4721 - t4964 + t4967 + t4969 + t4972 - t4975 - t4977 - t4979 + t4981 - t4984 + t4791 - t4794;
    let t4987 = t732 * t1385;
    (t4983, t4984, t4985, t4987)
}
