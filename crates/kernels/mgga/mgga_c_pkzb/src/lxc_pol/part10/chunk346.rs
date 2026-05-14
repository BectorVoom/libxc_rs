//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 346/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk346<F: Float>(t1134: F, t1144: F, t307: F, t311: F, t1072: F, t1086: F, t1112: F, t1114: F, t1118: F, t135: F, t273: F, t805: F) -> (F, F) {
    let t1147 = 0.65854491829355115987e0 * t1134 * t311 - 0.65854491829355115987e0 * t307 * t1144;
    let t1151 = t1147 * t135 * t273 * t805 - t1072 + t1086 + t1112 + t1114 - t1118;
    (t1147, t1151)
}
