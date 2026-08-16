//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 986/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk986(t1554: f64, t30540: f64, t1558: f64, t30137: f64, t7585: f64, t8525: f64, t1072: f64, t535: f64, t7507: f64, t7512: f64, t7447: f64, t8924: f64) -> (f64, f64, f64, f64, f64) {
    let t34853 = t30540 * t1554;
    let t34855 = t30540 * t1558;
    let t34856 = 0.40015750243531754508e-2_f64 * t34855;
    let t34865 = t7585 * t30137 * t8525;
    let t34866 = 0.14291339372689912324e-3_f64 * t34865;
    let t34879 = t7507 * t7512 * t535 * t1072;
    let t34893 = t7447 * t8924;
    (t34853, t34856, t34866, t34879, t34893)
}
