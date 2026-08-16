//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1330/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1330(t114452: f64, t28167: f64, t8717: f64, t6861: f64, t7910: f64, t108133: f64, t108135: f64, t108139: f64, t108141: f64, t108153: f64, t108156: f64, t108175: f64, t1903: f64, t2022: f64, t22974: f64, t23042: f64, t26079: f64, t27837: f64, t30032: f64, t30055: f64, t30082: f64, t4003: f64, t7295: f64, t7296: f64, t94409: f64, t94656: f64) -> (f64, f64, f64) {
    let t114455 = 18.0_f64 * t28167 * t8717 * t114452;
    let t114477 = t7910 * t6861;
    let t114484 = -0.21684070470512998656e-1_f64 * t108133 + 0.38554277296572111609e-1_f64 * t108135 + 0.8673628188205199462e0_f64 * t7295 * t7296 * t2022 * t23042 + 0.10408353825846239354e2_f64 * t7295 * t94656 * t2022 * t22974 + 0.26020884564615598386e1_f64 * t7295 * t7296 * t30055 * t1903 + 0.26020884564615598386e1_f64 * t27837 * t30032 + 0.86736281882051994623e-1_f64 * t108139 - 0.15421710918628844643e0_f64 * t108141 - t94409 + 0.77108554593144223218e-1_f64 * t108153 - 0.26020884564615598386e1_f64 * t27837 * t30082 - 0.26020884564615598386e1_f64 * t7295 * t26079 * t114477 * t4003 + 0.16463622957338778996e-1_f64 * t108156 + 0.29272321618148349057e-1_f64 * t108175;
    (t114455, t114477, t114484)
}
