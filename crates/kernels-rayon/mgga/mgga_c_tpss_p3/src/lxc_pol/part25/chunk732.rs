//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 732/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk732(t198: f64, t207: f64, t2224: f64, t2281: f64, t2285: f64, t2436: f64, t4680: f64, t4682: f64, t4685: f64, t4686: f64, t4687: f64, t4701: f64, t4742: f64, t4802: f64, t4806: f64, t740: f64, t823: f64) -> f64 {
    let t4810 = -t198 * t207 * t2436 * t4806 + t198 * t207 * t4802 * t823 + 3.0_f64 * t198 * t4701 * t740 + t2224 - t2281 - t2285 + t4680 + t4682 + t4685 - t4686 - t4687 + t4742;
    t4810
}
