//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 722/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk722<F: Float>(t1806: F, t8501: F, t1814: F, t7715: F, t7718: F, t696: F, t8494: F, t11625: F, t8626: F, t965: F, t8629: F, t970: F, t8632: F, t8620: F, t8623: F, t8640: F, t960: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t23074 = t1806 * t8501;
    let t23080 = t1814 * t7715;
    let t23096 = t1814 * t7718;
    let t23118 = t696 * t8494;
    let t23225 = t11625 * t7715;
    let t23229 = t965 * t8626;
    let t23231 = t970 * t8629;
    let t23234 = t970 * t8632;
    let t23236 = t970 * t8620;
    let t23238 = t965 * t8623;
    let t23249 = t960 * t8640;
    (t23074, t23080, t23096, t23118, t23225, t23229, t23231, t23234, t23236, t23238, t23249)
}
