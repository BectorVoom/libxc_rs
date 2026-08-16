//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1329/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1329<F: Float>(t10276: F, t698: F, t986: F, t973: F, t241: F, t625: F, t281: F, t283: F, t2403: F, t909: F, t2978: F, t2967: F, t964: F) -> (F, F, F, F, F, F, F, F) {
    let t10277 = F::cast_from(1.0_f64) / t10276;
    let t10286 = t698 * t986;
    let t10287 = t973 * t10286;
    let t10292 = t625 * t241;
    let t10294 = t281 * t10292 * t283;
    let t10295 = F::cast_from(20.0_f64) / F::cast_from(27.0_f64) * t10294;
    let t10296 = t2403 * t909;
    let t10304 = t241 * t2978;
    let t10333 = t964 * t2967;
    (t10277, t10287, t10292, t10294, t10295, t10296, t10304, t10333)
}
