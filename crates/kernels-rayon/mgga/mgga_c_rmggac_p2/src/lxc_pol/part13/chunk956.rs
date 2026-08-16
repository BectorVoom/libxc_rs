//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 956/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk956(t39666: f64, t7788: f64, t262: f64, t40833: f64, t36254: f64, t40805: f64, t7782: f64, t40808: f64, t35929: f64, t40738: f64, t4669: f64, t39688: f64, t5271: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40970 = t7788 * t39666;
    let t40972 = t262 * t40833;
    let t40973 = t36254 * t40972;
    let t40975 = t262 * t40805;
    let t40976 = t7782 * t40975;
    let t40978 = t262 * t40808;
    let t40979 = t35929 * t40978;
    let t40981 = t4669 * t40738;
    let t40991 = t5271 * t39688;
    (t40970, t40972, t40973, t40975, t40976, t40978, t40979, t40981, t40991)
}
