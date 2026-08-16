//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1319/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1319<F: Float>(t29506: F, t7901: F, t1907: F, t6836: F, t28167: F, t8717: F, t6861: F, t7910: F, t108133: F, t108135: F, t108139: F, t108141: F, t108153: F, t108156: F, t108175: F, t1903: F, t2022: F, t22974: F, t23042: F, t26079: F, t27837: F, t30032: F, t30055: F, t30082: F, t4003: F, t7295: F, t7296: F, t94409: F, t94656: F) -> (F, F, F, F) {
    let t114451 = F::cast_from(9.0_f64) * t29506 * t7901;
    let t114452 = t6836 * t1907;
    let t114455 = F::cast_from(18.0_f64) * t28167 * t8717 * t114452;
    let t114477 = t7910 * t6861;
    let t114484 = -F::cast_from(0.21684070470512998656e-1_f64) * t108133 + F::cast_from(0.38554277296572111609e-1_f64) * t108135 + F::cast_from(0.8673628188205199462e0_f64) * t7295 * t7296 * t2022 * t23042 + F::cast_from(0.10408353825846239354e2_f64) * t7295 * t94656 * t2022 * t22974 + F::cast_from(0.26020884564615598386e1_f64) * t7295 * t7296 * t30055 * t1903 + F::cast_from(0.26020884564615598386e1_f64) * t27837 * t30032 + F::cast_from(0.86736281882051994623e-1_f64) * t108139 - F::cast_from(0.15421710918628844643e0_f64) * t108141 - t94409 + F::cast_from(0.77108554593144223218e-1_f64) * t108153 - F::cast_from(0.26020884564615598386e1_f64) * t27837 * t30082 - F::cast_from(0.26020884564615598386e1_f64) * t7295 * t26079 * t114477 * t4003 + F::cast_from(0.16463622957338778996e-1_f64) * t108156 + F::cast_from(0.29272321618148349057e-1_f64) * t108175;
    (t114451, t114455, t114477, t114484)
}
