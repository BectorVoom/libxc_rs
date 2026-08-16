//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 999/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk999(t10772: f64, t237: f64, t10779: f64, t10782: f64, t10785: f64, t10870: f64, t10894: f64, t10896: f64, t10898: f64, t10900: f64, t10903: f64, t10921: f64, t10930: f64) -> (f64, f64) {
    let t10977 = 0.19751673498613801407e-1_f64 * t237 * t10772;
    let t10978 = t10894 - t10782 + t10785 - t10779 + t10977 + t10896 + t10898 + t10900 - t10903 + t10921 + t10870 + t10930;
    (t10977, t10978)
}
