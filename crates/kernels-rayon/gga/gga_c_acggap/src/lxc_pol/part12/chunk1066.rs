//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1066/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1066(t1181: f64, t20992: f64, t7351: f64, t7426: f64, t20138: f64, t599: f64, t7413: f64, t33735: f64, t1983: f64, t30127: f64, t7586: f64, t8791: f64) -> (f64, f64, f64, f64) {
    let t34945 = t7426 * t1181 * t7351 * t20992;
    let t34949 = t7413 * t1181 * t599 * t20138;
    let t34953 = t7413 * t1181 * t599 * t33735;
    let t34957 = t30127 * t7586 * t1983 * t8791;
    (t34945, t34949, t34953, t34957)
}
