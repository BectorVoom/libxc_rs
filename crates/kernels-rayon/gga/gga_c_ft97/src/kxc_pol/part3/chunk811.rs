//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 811/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk811(t4893: f64, t648: f64, t3664: f64, t3659: f64, t920: f64, t363: f64, t4894: f64, t4889: f64, t4844: f64, t5: f64, t1080: f64, t13273: f64, t2240: f64, t3601: f64, t3660: f64, t3665: f64, t3668: f64, t4890: f64, t4895: f64, t4898: f64, t623: f64, t650: f64) -> f64 {
    let t16585 = t4893 * t648;
    let t16586 = t16585 * t3664;
    let t16591 = t3659 * t920;
    let t16594 = t4894 * t363;
    let t16601 = t4889 * t363;
    let t16612 = t5 * t4844;
    let t16615 = t623 * t16586 / 4.0_f64 + t3601 * t3665 / 2.0_f64 + t623 * t16591 / 2.0_f64 + t623 * t16594 / 4.0_f64 + t2240 * t4898 / 2.0_f64 + t13273 * t1080 / 2.0_f64 + t623 * t16601 / 4.0_f64 + t2240 * t4895 / 4.0_f64 + t3601 * t3660 / 2.0_f64 + t3601 * t3668 / 2.0_f64 + t2240 * t4890 / 4.0_f64 + t16612 * t650 / 4.0_f64;
    t16615
}
