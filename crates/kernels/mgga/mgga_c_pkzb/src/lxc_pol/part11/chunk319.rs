//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 319/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk319<F: Float>(t1116: F, t730: F, t1066: F, t154: F, t742: F, t1072: F, t1086: F, t1112: F, t1114: F) -> (F, F, F) {
    let t1118 = F::new(0.5848223622634646207e0) * t730 * t1116;
    let t1120 = t154 * t742 * t1066;
    let t1123 = -t1072 + t1086 + t1112 + t1114 - t1118;
    (t1118, t1120, t1123)
}
