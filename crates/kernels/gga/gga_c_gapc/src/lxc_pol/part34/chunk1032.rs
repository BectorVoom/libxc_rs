//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 1032/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk1032<F: Float>(t11971: F, t277: F, t34021: F, t33521: F, t4052: F, t1084: F, t29868: F, t10079: F, t33620: F, t11849: F, t1952: F, t919: F, t11761: F, t34005: F, t3775: F, t9586: F) -> (F, F, F, F, F, F, F) {
    let t34023 = t277 * t34021 * t11971;
    let t34026 = t4052 * t33521 * M_PI;
    let t34028 = t1084 * t34026 * t29868;
    let t34030 = t33620 * t10079;
    let t34033 = t11849 * t1952 * t919;
    let t34036 = t34005 * t11761;
    let t34038 = t3775 * t9586;
    (t34023, t34026, t34028, t34030, t34033, t34036, t34038)
}
