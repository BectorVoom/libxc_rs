//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1157/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1157(t36986: f64, t42830: f64, t1065: f64, t2530: f64, t3270: f64, t3579: f64, t3060: f64, t36967: f64, t3269: f64, t10615: f64, t12395: f64, t3262: f64) -> (f64, f64, f64, f64) {
    let t42832 = 3.0_f64 / 2.0_f64 * t36986 * t42830;
    let t42836 = t3579 * t3270 * t1065 * t2530 / 2.0_f64;
    let t42837 = t1065 * t3060;
    let t42838 = t36967 * t42837;
    let t42840 = 45.0_f64 / 64.0_f64 * t3269 * t42838;
    let t42843 = 15.0_f64 / 8.0_f64 * t3262 * t10615 * t12395;
    (t42832, t42836, t42840, t42843)
}
