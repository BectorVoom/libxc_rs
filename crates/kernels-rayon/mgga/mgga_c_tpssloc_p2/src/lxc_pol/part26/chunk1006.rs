//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1006/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1006(t1174: f64, t11761: f64, t11766: f64, t11770: f64, t11774: f64, t11781: f64, t11787: f64, t11792: f64, t11794: f64, t11798: f64, t11802: f64, t11805: f64, t11809: f64, t11814: f64, t1218: f64, t1227: f64, t3515: f64) -> f64 {
    let t11817 = t1174 * t11761 / 36.0_f64 - 7.0_f64 / 648.0_f64 * t1174 * t11766 - t3515 * t11770 / 1024.0_f64 + 5.0_f64 / 4608.0_f64 * t1227 * t11774 - 5.0_f64 / 5184.0_f64 * t1227 * t11781 + 5.0_f64 / 6912.0_f64 * t11787 + t11792 / 6912.0_f64 + t11794 / 768.0_f64 - t11798 / 2304.0_f64 - t11802 / 1152.0_f64 - t1227 * t11805 / 4608.0_f64 - t1227 * t11809 / 768.0_f64 + t11814 * t1218 / 1024.0_f64;
    t11817
}
