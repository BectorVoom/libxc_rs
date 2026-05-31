//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 649/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk649<F: Float>(t1163: F, t6351: F, t175: F, t4352: F, t5641: F, t1846: F, t952: F, t1531: F, t3396: F, t3403: F, t367: F, t3741: F, t3745: F, t3752: F, t3757: F, t3762: F, t3766: F, t3772: F, t3777: F, t3782: F, t418: F, t4463: F, t4735: F, t6320: F, t6324: F, t6328: F, t6332: F, t6335: F, t6339: F, t6343: F, t6348: F) -> (F, F) {
    let t6352 = t1163 * t6351;
    let t6361 = t4352 * t175 * t5641;
    let t6364 = t952 * t1846;
    let t6366 = t367 * t6320 / F::cast_from(24.0_f64) - F::cast_from(0.17149607247227894789e-1_f64) * t4463 * t6324 + F::cast_from(0.68598428988911579156e-2_f64) * t3396 * t6328 + F::cast_from(0.42874018118069736972e-3_f64) * t1531 * t6332 - F::cast_from(0.40015750243531754507e-2_f64) * t6335 + F::cast_from(0.51448821741683684367e-2_f64) * t4735 * t6339 - F::cast_from(0.42874018118069736972e-2_f64) * t3403 * t6343 + F::cast_from(0.34299214494455789578e-2_f64) * t6348 + F::cast_from(0.21437009059034868486e-3_f64) * t6352 + F::cast_from(0.20007875121765877254e-2_f64) * t3741 - F::cast_from(0.40015750243531754508e-2_f64) * t3745 + F::cast_from(0.40015750243531754508e-2_f64) * t3752 - F::cast_from(0.42874018118069736972e-3_f64) * t3757 + F::cast_from(0.42874018118069736972e-3_f64) * t3762 - F::cast_from(0.56688979511669985553e-2_f64) * t3766 - F::cast_from(0.25724410870841842183e-1_f64) * t418 * t6361 + F::cast_from(0.10003937560882938627e-2_f64) * t6364 + t3772 + t3777 + t3782;
    (t6361, t6366)
}
