//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 946/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk946(t10973: f64, t2300: f64, t265: f64, t267: f64, t10645: f64) -> (f64, f64, f64, f64) {
    let t10974 = 0.30487649791575028314e-3_f64 * t10973;
    let t10976 = t2300 * t265;
    let t10977 = t10976 * t267;
    let t10978 = t10645 * t10977;
    (t10974, t10976, t10977, t10978)
}
