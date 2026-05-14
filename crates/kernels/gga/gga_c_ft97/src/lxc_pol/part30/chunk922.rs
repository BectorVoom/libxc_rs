//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 922/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk922<F: Float>(t150094: F, t150150: F, t150201: F, t150253: F, t150285: F, t150948: F, t150998: F, t151047: F, t762: F, t1173: F, t7484: F, t1403: F, t140710: F, t140712: F, t141406: F, t150023: F, t150031: F, t193: F, t2354: F, t27939: F, t28020: F, t33499: F, t33568: F, t35744: F, t4003: F, t6002: F, t684: F, t6844: F, t719: F, t7437: F, t7485: F) -> (F, F) {
    let t151051 = t762 * (t150094 + t150150 + t150201 + t150253 + t150285 + t150948 + t150998 + t151047);
    let t151053 = t7484 * t1173;
    let t151065 = t140710 / 9.0 - t150023 / 18.0 + t140712 / 9.0 - t719 * t35744 + t1403 * t193 * t7485 * t4003 / 6.0 - 4.0 * t150031 - 2.0 * t151051 - t6002 * t2354 * t151053 * t684 / 18.0 + t7437 * t27939 / 6.0 - t141406 / 27.0 + t33568 * t6844 / 6.0 - t33499 * t28020 / 18.0;
    (t151051, t151065)
}
