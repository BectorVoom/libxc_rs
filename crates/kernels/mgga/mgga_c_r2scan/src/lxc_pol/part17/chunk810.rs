//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 810/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk810<F: Float>(t2147: F, t8232: F, t1591: F, t2666: F, t1568: F, t8089: F, t7623: F, t2214: F, t2698: F, t514: F, t1616: F, t938: F) -> (F, F, F, F, F, F) {
    let t8234 = F::cast_from(0.11643651550782197811e-1_f64) * t2147 * t8232;
    let t8240 = t1591 * t2666;
    let t8243 = t1568 * t8089;
    let t8245 = F::cast_from(0.10975748638225852664e-1_f64) * t7623 * t8243;
    let t8263 = t2214 * t2698;
    let t8265 = F::cast_from(0.19514881078765566037e-1_f64) * t514 * t8263;
    let t8266 = t1616 * t938;
    (t8234, t8240, t8243, t8245, t8265, t8266)
}
