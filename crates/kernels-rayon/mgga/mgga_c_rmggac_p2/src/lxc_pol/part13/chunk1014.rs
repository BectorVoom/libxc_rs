//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1014/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1014(t16156: f64, t9194: f64, t9190: f64, t1001: f64, t236: f64, t3351: f64, t35312: f64, t551: f64, t27111: f64, t515: f64, t9188: f64, t9184: f64) -> (f64, f64, f64, f64, f64) {
    let t42204 = t16156 * t9194;
    let t42206 = t16156 * t9190;
    let t42211 = t3351 * t35312 * t236 * t551 * t1001;
    let t42215 = t3351 * t9188 * t515 * t27111;
    let t42217 = t16156 * t9184;
    (t42204, t42206, t42211, t42215, t42217)
}
