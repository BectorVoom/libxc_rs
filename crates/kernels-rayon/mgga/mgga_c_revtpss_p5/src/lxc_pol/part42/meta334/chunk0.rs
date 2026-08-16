//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1131/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1131(t10726: f64, t14868: f64, t2661: f64, t10868: f64, t241: f64, t820: f64, t10811: f64, t4452: f64, t2719: f64, t844: f64, t4368: f64, t2482: f64, t814: f64) -> (f64, f64, f64, f64, f64) {
    let t14869 = t10726 * t14868;
    let t14871 = 0.28582678745379824648e-4_f64 * t2661 * t14869;
    let t14894 = t820 * t10868 * t241;
    let t14907 = t10811 * t4452;
    let t14923 = t820 * t2719 * t844;
    let t14925 = 0.40015750243531754508e-2_f64 * t14923 * t4368;
    let t14931 = t2482 * t2719 * t814;
    (t14871, t14894, t14907, t14925, t14931)
}
