//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 781/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk781(t1224: f64, t5381: f64, t774: f64, t1625: f64, t520: f64, t4416: f64, t3273: f64, t2281: f64, t2285: f64, t2292: f64, t2302: f64, t2310: f64, t3189: f64, t3209: f64, t3281: f64, t3304: f64, t5347: f64, t5348: f64) -> (f64, f64, f64, f64) {
    let t5383 = t1224 * t774 * t5381;
    let t5387 = t520 * t1625;
    let t5388 = t4416 * t5387;
    let t5389 = t3273 * t5388;
    let t5392 = t2302 + t2310 - t2292 - t2281 - t2285 + t3281 - t3209 - t5348 - t5347 + t3189 - t3304;
    (t5383, t5387, t5389, t5392)
}
