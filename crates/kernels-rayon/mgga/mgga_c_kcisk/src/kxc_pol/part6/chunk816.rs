//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 816/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk816(t14090: f64, t7706: f64, t1346: f64, t8111: f64, t1391: f64, t8099: f64, t443: f64, t8105: f64, t8108: f64, t7737: f64, t821: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t25544 = t14090 * t7706;
    let t25557 = t1346 * t8111;
    let t25559 = t1391 * t8099;
    let t25561 = t443 * t8105;
    let t25563 = t1346 * t8108;
    let t25590 = t821 * t7737;
    (t25544, t25557, t25559, t25561, t25563, t25590)
}
