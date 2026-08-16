//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 743/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk743(t17198: f64, t925: f64, t2210: f64, t20748: f64, t3434: f64, t2221: f64, t20753: f64, t9127: f64, t1053: f64, t17409: f64, t144: f64, t1017: f64, t4839: f64, t574: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t20926 = t17198 * t925;
    let t20927 = t2210 * t20926;
    let t20930 = t3434 * t20748;
    let t20931 = t2221 * t20930;
    let t20934 = t9127 * t20753;
    let t20935 = t2210 * t20934;
    let t20938 = t17409 * t1053;
    let t20939 = t144 * t20938;
    let t20942 = t574 * t4839 * t1017;
    (t20926, t20927, t20930, t20931, t20934, t20935, t20938, t20939, t20942)
}
