//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1054/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1054(t150094: f64, t150150: f64, t150201: f64, t150253: f64, t150285: f64, t150948: f64, t150998: f64, t151047: f64, t762: f64, t1173: f64, t7484: f64, t1403: f64, t140710: f64, t140712: f64, t141406: f64, t150023: f64, t150031: f64, t193: f64, t2354: f64, t27939: f64, t28020: f64, t33499: f64, t33568: f64, t35744: f64, t4003: f64, t6002: f64, t684: f64, t6844: f64, t719: f64, t7437: f64, t7485: f64) -> (f64, f64) {
    let t151051 = t762 * (t150094 + t150150 + t150201 + t150253 + t150285 + t150948 + t150998 + t151047);
    let t151053 = t7484 * t1173;
    let t151065 = t140710 / 9.0_f64 - t150023 / 18.0_f64 + t140712 / 9.0_f64 - t719 * t35744 + t1403 * t193 * t7485 * t4003 / 6.0_f64 - 4.0_f64 * t150031 - 2.0_f64 * t151051 - t6002 * t2354 * t151053 * t684 / 18.0_f64 + t7437 * t27939 / 6.0_f64 - t141406 / 27.0_f64 + t33568 * t6844 / 6.0_f64 - t33499 * t28020 / 18.0_f64;
    (t151051, t151065)
}
