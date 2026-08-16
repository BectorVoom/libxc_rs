//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 824/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk824(t1391: f64, t574: f64, t5842: f64, t1882: f64, t7402: f64, t2142: f64, t7357: f64, t7350: f64, t1359: f64, t5975: f64, t616: f64, t7339: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33151 = t574 * t1391 * t5842;
    let t33155 = 2.0_f64 / 9.0_f64 * t1882 * t7402;
    let t33157 = t574 * t2142 * t7357;
    let t33161 = 2.0_f64 / 9.0_f64 * t1882 * t7350;
    let t33163 = t574 * t5975 * t1359;
    let t33167 = t574 * t616 * t7339;
    (t33151, t33155, t33157, t33161, t33163, t33167)
}
