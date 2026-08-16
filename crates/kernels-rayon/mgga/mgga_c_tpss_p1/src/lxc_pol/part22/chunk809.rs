//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 809/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk809(t3: f64, t4543: f64, t116: f64, t1338: f64, t645: f64, t117: f64, t3537: f64, t1279: f64, t1281: f64, t1668: f64, t1670: f64, t547: f64, t548: f64, param_d: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4544 = t3 * t4543;
    let t4549 = param_d * t4543;
    let t4555 = t116 * t1338;
    let t4556 = t4555 * t645;
    let t4559 = t117 * t3537;
    let t4562 = 3.0_f64 * t1279 * t1670 + 3.0_f64 * t1281 * t1668 + t4549 * t548 + 6.0_f64 * t4556 * t547 + 3.0_f64 * t4559 * t547;
    (t4544, t4549, t4555, t4556, t4559, t4562)
}
