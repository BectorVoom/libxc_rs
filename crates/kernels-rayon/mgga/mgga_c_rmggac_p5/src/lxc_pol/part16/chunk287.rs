//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 287/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk287(t1587: f64, t1614: f64, t1624: f64, t1627: f64, t1632: f64, t1635: f64, t305: f64, t326: f64, t344: f64, t349: f64, t793: f64, t797: f64, t838: f64, t851: f64, t854: f64, t861: f64) -> f64 {
    let t1652 = 0.39914139006212695214e-1_f64 * t793 * t1624 - 0.59871208509319042821e-1_f64 * t797 * t1627 + 0.19957069503106347607e-1_f64 * t305 * t1587 - 0.59871208509319042821e-1_f64 * t797 * t1632 + 0.79828278012425390428e-1_f64 * t838 * t1635 - 0.19957069503106347607e-1_f64 * t326 * t1614 + 0.13276154105060581339e-2_f64 * t851 * t1624 - 0.15931384926072697607e-2_f64 * t854 * t1627 + 0.26552308210121162678e-3_f64 * t344 * t1587 - 0.15931384926072697607e-2_f64 * t854 * t1632 + 0.18586615747084813875e-2_f64 * t861 * t1635 - 0.26552308210121162678e-3_f64 * t349 * t1614;
    t1652
}
