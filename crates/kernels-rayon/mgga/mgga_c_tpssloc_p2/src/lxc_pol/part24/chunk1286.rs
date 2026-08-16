//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 1286/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk1286(t12240: f64, t12251: f64, t12255: f64, t1336: f64, t1352: f64, t22709: f64, t31201: f64, t5334: f64, t5344: f64, t81184: f64, t81187: f64, t81189: f64, t81193: f64, t81197: f64, t81199: f64, t81203: f64, t81209: f64, t81213: f64, t81216: f64, t81218: f64, t81222: f64, t81225: f64, t81230: f64, t81234: f64, t81238: f64, t81243: f64) -> f64 {
    let t81250 = -0.11514538467937585055e0_f64 * t81184 - 0.38381794893125283518e0_f64 * t81187 + 0.23029076935875170111e0_f64 * t81189 + 0.14804406601634037928e0_f64 * t81193 + 0.49348022005446793095e-1_f64 * t81197 - 3.0_f64 * t1336 * t81199 * t1352 - 3.0_f64 * t5344 * t81203 * t1352 - 0.49348022005446793095e-1_f64 * t81209 - 0.16449340668482264365e-1_f64 * t81213 + 0.24674011002723396548e-1_f64 * t81216 + 0.11514538467937585055e0_f64 * t81218 - 0.9869604401089358619e-1_f64 * t81222 - 0.24674011002723396548e-1_f64 * t81225 - 0.49348022005446793095e-1_f64 * t81230 + 0.9869604401089358619e-1_f64 * t81234 + 0.49348022005446793095e-1_f64 * t81238 + 6.0_f64 * t5334 * t31201 * t12240 - 6.0_f64 * t1336 * t81243 * t12251 + 6.0_f64 * t1336 * t22709 * t12255;
    t81250
}
