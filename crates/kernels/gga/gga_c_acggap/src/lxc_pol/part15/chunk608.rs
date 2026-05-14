//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 608/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk608<F: Float>(t1350: F, t530: F, t1181: F, t3361: F, t1165: F, t1539: F, t5862: F, t1163: F, t175: F, t4352: F, t5641: F, t1846: F, t952: F, t1531: F, t3396: F, t3403: F, t367: F, t3741: F, t3745: F, t3752: F, t3757: F, t3762: F, t3766: F, t3772: F, t3777: F, t3782: F, t418: F, t4463: F, t4735: F, t6320: F, t6324: F, t6328: F, t6332: F, t6335: F, t6339: F, t6343: F) -> (F, F, F, F) {
    let t6346 = t530 * t1350;
    let t6347 = t1181 * t6346;
    let t6348 = t3361 * t6347;
    let t6351 = t1165 * t5862 * t1539;
    let t6352 = t1163 * t6351;
    let t6361 = t4352 * t175 * t5641;
    let t6364 = t952 * t1846;
    let t6366 = t367 * t6320 / 24.0 - 0.17149607247227894789e-1 * t4463 * t6324 + 0.68598428988911579156e-2 * t3396 * t6328 + 0.42874018118069736972e-3 * t1531 * t6332 - 0.40015750243531754507e-2 * t6335 + 0.51448821741683684367e-2 * t4735 * t6339 - 0.42874018118069736972e-2 * t3403 * t6343 + 0.34299214494455789578e-2 * t6348 + 0.21437009059034868486e-3 * t6352 + 0.20007875121765877254e-2 * t3741 - 0.40015750243531754508e-2 * t3745 + 0.40015750243531754508e-2 * t3752 - 0.42874018118069736972e-3 * t3757 + 0.42874018118069736972e-3 * t3762 - 0.56688979511669985553e-2 * t3766 - 0.25724410870841842183e-1 * t418 * t6361 + 0.10003937560882938627e-2 * t6364 + t3772 + t3777 + t3782;
    (t6347, t6351, t6361, t6366)
}
