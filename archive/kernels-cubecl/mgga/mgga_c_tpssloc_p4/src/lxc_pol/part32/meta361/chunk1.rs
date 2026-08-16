//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1412/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1412<F: Float>(t1174: F, t15285: F, t11583: F, t3961: F, t11529: F, t1709: F, t3432: F, t4889: F, t3450: F, t3966: F, t3448: F, t4928: F) -> (F, F, F, F, F, F) {
    let t15287 = F::cast_from(0.18518518518518518518e-3_f64) * t1174 * t15285;
    let t15293 = t11583 * t3961;
    let t15299 = t11529 * t1709;
    let t15300 = t1174 * t15299;
    let t15307 = t4889 * t3432;
    let t15313 = t3450 * t3966;
    let t15320 = t3448 * t4928;
    (t15287, t15293, t15300, t15307, t15313, t15320)
}
