//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 680/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk680(t3005: f64, t3295: f64, t9800: f64, t11053: f64, t9805: f64, t1029: f64, t9796: f64, t3247: f64, t900: f64, t10867: f64, t10924: f64, t787: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13052 = t3005 * t3295;
    let t13053 = t9800 * t13052;
    let t13055 = t11053 * t3295;
    let t13056 = t9805 * t13055;
    let t13058 = t1029 * t3295;
    let t13059 = t9796 * t13058;
    let t13072 = t900 * t3247;
    let t13073 = t10867 * t13072;
    let t13077 = t787 * t10924;
    (t13052, t13053, t13055, t13056, t13058, t13059, t13072, t13073, t13077)
}
