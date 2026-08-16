//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1236/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1236(t237: f64, t30377: f64, t1084: f64, t26283: f64, t30242: f64, t30245: f64, t30248: f64, t30252: f64, t30255: f64, t30259: f64, t30261: f64, t30263: f64, t30265: f64, t30268: f64, t30270: f64, t30273: f64, t30275: f64, t30277: f64, t30362: f64, t30364: f64, t30366: f64, t30369: f64) -> (f64, f64, f64) {
    let t30379 = 0.19751673498613801407e-1_f64 * t237 * t30377;
    let t30381 = 3.0_f64 * t26283 * t1084;
    let t30382 = -t30242 - t30245 - t30248 + t30252 + t30255 + t30259 + t30261 - t30263 - t30265 - t30268 - t30270 + t30273 - t30275 - t30277 + t30362 + t30364 + t30366 + t30369 + t30379 + t30381;
    (t30379, t30381, t30382)
}
