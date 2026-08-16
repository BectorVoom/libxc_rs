//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 936/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk936(t13648: f64, t2194: f64, t1445: f64, t35558: f64, t813: f64, t935: f64, t13621: f64, t5782: f64, t36390: f64, t787: f64, t9824: f64, t10914: f64, t2365: f64, t35446: f64) -> (f64, f64, f64, f64, f64) {
    let t45894 = 0.46011511144704899612e1_f64 * t2194 * t13648;
    let t45898 = 0.46011511144704899612e1_f64 * t813 * t1445 * t35558 * t935;
    let t45900 = 0.69017266717057349418e1_f64 * t5782 * t13621;
    let t45902 = t787 * t36390 * t9824;
    let t45903 = 0.14896037479937677779e-1_f64 * t45902;
    let t45905 = t10914 * t2365 * t35446;
    (t45894, t45898, t45900, t45903, t45905)
}
