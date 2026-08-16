//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 982/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk982(t14889: f64, t319: f64, t840: f64, t1091: f64, t2867: f64, t10703: f64, t2770: f64, t14690: f64, t4311: f64, t684: f64, t835: f64, t4246: f64) -> (f64, f64, f64, f64, f64) {
    let t15222 = t840 * t319 * t14889;
    let t15225 = t1091 * t2867;
    let t15226 = t10703 * t15225;
    let t15229 = t2770 * t319;
    let t15230 = t15229 * t14690;
    let t15234 = t835 * t4311 * t684;
    let t15238 = t840 * t4246 * t2867;
    (t15222, t15226, t15230, t15234, t15238)
}
