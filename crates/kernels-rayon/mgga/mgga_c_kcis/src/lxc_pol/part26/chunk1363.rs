//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1363/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1363(t3717: f64, t3754: f64, t103101: f64, t5701: f64, t103239: f64, t7908: f64, t28372: f64, t52697: f64, t5885: f64, t16937: f64, t29258: f64, t102158: f64, t102180: f64, t102183: f64, t12194: f64, t16901: f64, t16906: f64, t20984: f64, t21655: f64, t27369: f64, t27438: f64, t59414: f64, t94408: f64, t94519: f64) -> (f64, f64) {
    let t103372 = t3717 * t3754;
    let t103374 = t5701 * t103372 * t103101;
    let t103391 = t7908 * t103239;
    let t103394 = t28372 * t5885 * t52697;
    let t103399 = t7908 * t16937 * t29258;
    let t103402 = -0.55273148148148148147e-2_f64 * t102158 + 0.30891203703703703704e-3_f64 * t7908 * t12194 * t27438 * t59414 + 0.30891203703703703704e-3_f64 * t7908 * t103374 + 0.18534722222222222223e-2_f64 * t7908 * t5701 * t94519 * t20984 + 0.41224311342592592593e-4_f64 * t27369 * t103374 - 0.72079475308641975309e-3_f64 * t7908 * t16901 * t94408 * t20984 + 0.12356481481481481482e-2_f64 * t7908 * t16906 * t27438 * t21655 + 0.15445601851851851852e-3_f64 * t103391 + 0.37101880208333333334e-3_f64 * t27369 * t103394 - 0.22109259259259259259e-2_f64 * t102180 + 0.15445601851851851852e-3_f64 * t103399 - 0.33163888888888888888e-2_f64 * t102183;
    (t103394, t103402)
}
