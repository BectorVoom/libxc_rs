//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1069/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1069(t4840: f64, t570: f64, t1432: f64, t1992: f64, t30147: f64, t7586: f64, t1494: f64, t7329: f64, t1498: f64, t30716: f64, t500: f64, t1181: f64, t2068: f64, t5080: f64, t599: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35001 = t570 * t4840;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35039 = t7329 * t1494;
    let t35041 = t7329 * t1498;
    let t35043 = t30716 * t500;
    let t35047 = t2068 * t1181 * t599 * t5080;
    (t35001, t35022, t35039, t35041, t35043, t35047)
}
