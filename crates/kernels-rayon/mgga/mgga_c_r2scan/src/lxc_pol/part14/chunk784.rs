//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 784/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk784(t1266: f64, t359: f64, t259: f64, t2298: f64, t363: f64, t364: f64, t358: f64) -> (f64, f64, f64) {
    let t6848 = t359 * t1266;
    let t6849 = t259 * t6848;
    let t6852 = t2298 * t363;
    let t6854 = 1.0_f64 / t364 / t6852;
    let t6855 = t358 * t6854;
    (t6849, t6854, t6855)
}
