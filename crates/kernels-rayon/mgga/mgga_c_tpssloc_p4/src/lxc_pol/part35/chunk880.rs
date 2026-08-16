//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 880/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk880(t12248: f64, t562: f64, t3792: f64, t550: f64, t1339: f64, t836: f64, t1336: f64, t236: f64, t240: f64, t10022: f64, t248: f64, t557: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12249 = t12248 * t562;
    let t12250 = t3792 * t550;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12289 = t12248 * t236;
    let t12290 = t12289 * t240;
    let t12291 = t1336 * t12290;
    let t12328 = t10022 * t557 * t248;
    (t12249, t12250, t12283, t12289, t12291, t12328)
}
