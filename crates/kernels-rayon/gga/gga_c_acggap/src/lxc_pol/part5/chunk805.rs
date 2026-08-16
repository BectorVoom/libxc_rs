//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 805/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk805(t175: f64, t4352: f64, t5641: f64, t1846: f64, t952: f64, t1531: f64, t3396: f64, t3403: f64, t367: f64, t3741: f64, t3745: f64, t3752: f64, t3757: f64, t3762: f64, t3766: f64, t3772: f64, t3777: f64, t3782: f64, t418: f64, t4463: f64, t4735: f64, t6320: f64, t6324: f64, t6328: f64, t6332: f64, t6335: f64, t6339: f64, t6343: f64, t6348: f64, t6352: f64) -> (f64, f64) {
    let t6361 = t4352 * t175 * t5641;
    let t6364 = t952 * t1846;
    let t6366 = t367 * t6320 / 24.0_f64 - 0.17149607247227894789e-1_f64 * t4463 * t6324 + 0.68598428988911579156e-2_f64 * t3396 * t6328 + 0.42874018118069736972e-3_f64 * t1531 * t6332 - 0.40015750243531754507e-2_f64 * t6335 + 0.51448821741683684367e-2_f64 * t4735 * t6339 - 0.42874018118069736972e-2_f64 * t3403 * t6343 + 0.34299214494455789578e-2_f64 * t6348 + 0.21437009059034868486e-3_f64 * t6352 + 0.20007875121765877254e-2_f64 * t3741 - 0.40015750243531754508e-2_f64 * t3745 + 0.40015750243531754508e-2_f64 * t3752 - 0.42874018118069736972e-3_f64 * t3757 + 0.42874018118069736972e-3_f64 * t3762 - 0.56688979511669985553e-2_f64 * t3766 - 0.25724410870841842183e-1_f64 * t418 * t6361 + 0.10003937560882938627e-2_f64 * t6364 + t3772 + t3777 + t3782;
    (t6361, t6366)
}
