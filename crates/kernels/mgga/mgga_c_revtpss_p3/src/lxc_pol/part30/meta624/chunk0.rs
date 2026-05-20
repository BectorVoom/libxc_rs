//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2150/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2150<F: Float>(t99021: F, t4452: F, t92951: F, t14719: F, t25227: F, t2661: F, t14723: F, t14774: F, t7045: F, t25266: F, t4426: F, t1561: F, t93048: F) -> (F, F, F, F, F, F, F) {
    let t99022 = F::cast_from(0.50820002809285328226e-4_f64) * t99021;
    let t99023 = t92951 * t4452;
    let t99024 = F::cast_from(0.16006300097412701803e-1_f64) * t99023;
    let t99026 = t2661 * t25227 * t14719;
    let t99027 = F::cast_from(0.11433071498151929859e-3_f64) * t99026;
    let t99029 = t2661 * t25227 * t14723;
    let t99030 = F::cast_from(0.28582678745379824648e-4_f64) * t99029;
    let t99031 = t7045 * t14774;
    let t99033 = t25266 * t4426;
    let t99034 = F::cast_from(0.40015750243531754508e-2_f64) * t99033;
    let t99035 = t93048 * t1561;
    (t99022, t99024, t99027, t99030, t99031, t99034, t99035)
}
