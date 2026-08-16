//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 507/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk507(t3886: f64, t3892: f64, t3891: f64, t1131: f64, t258: f64, t684: f64, t2599: f64, t1154: f64, t2475: f64, t747: f64, t91: f64, t1148: f64, t1775: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3893 = t3892 * t3886;
    let t3894 = t3891 * t3893;
    let t3897 = t258 * t1131;
    let t3898 = t3897 * t684;
    let t3899 = t2599 * t3898;
    let t3902 = t2475 * t1154;
    let t3904 = t91 * t3902 * t747;
    let t3908 = t1775 * t1148;
    (t3893, t3894, t3898, t3899, t3902, t3904, t3908)
}
