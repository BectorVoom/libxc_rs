//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 804/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk804(t13052: f64, t28673: f64, t23000: f64, t33308: f64, t9889: f64, t11068: f64, t2617: f64, t7803: f64, t10867: f64, t1423: f64, t3247: f64, t13077: f64, t28439: f64) -> (f64, f64, f64, f64, f64) {
    let t43760 = t28673 * t13052;
    let t43832 = t23000 * t33308 * t9889;
    let t43881 = t7803 * t11068 * t2617;
    let t43907 = t10867 * t1423 * t3247;
    let t43912 = t13077 * t28439;
    (t43760, t43832, t43881, t43907, t43912)
}
