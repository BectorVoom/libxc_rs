//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1008/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1008(t1338: f64, t3879: f64, t3773: f64, t68: f64, t1339: f64, t836: f64, t1336: f64, t3809: f64, t12248: f64, t236: f64, t3777: f64, t3798: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12259 = t1338 * t3879;
    let t12267 = t3773 * t68;
    let t12282 = t1339 * t836;
    let t12283 = t1336 * t12282;
    let t12284 = t12283 * t3809;
    let t12289 = t12248 * t236;
    let t12300 = t3777 * t3798;
    (t12259, t12267, t12283, t12284, t12289, t12300)
}
