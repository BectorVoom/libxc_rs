//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1023/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1023<F: Float>(t19107: F, t22971: F, t19116: F, t54: F, t8253: F, t1167: F, t179: F, t19150: F, t404: F, t2411: F, t2888: F, t154: F, t3026: F, t385: F, t6446: F, t1220: F, t6448: F) -> (F, F, F, F, F, F, F) {
    let t23075 = t19107 * t22971;
    let t23081 = t19116 * t22971;
    let t23213 = t54 * t8253;
    let t23272 = t404 * t179 * t19150 * t1167;
    let t23278 = t2888 * t2411;
    let t23317 = t385 * t154 * t6446 * t3026;
    let t23318 = t23317 / 144.0;
    let t23331 = t1220 * t6448;
    (t23075, t23081, t23213, t23272, t23278, t23318, t23331)
}
