//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 751/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk751(t15746: f64, t1642: f64, t3051: f64, t4459: f64, t458: f64, t363: f64, t4454: f64) -> (f64, f64, f64) {
    let t15747 = t1642 * t15746;
    let t15748 = t3051 * t15747;
    let t15750 = t458 * t4459;
    let t15752 = t4454 * t363;
    (t15748, t15750, t15752)
}
