//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 941/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk941(t11083: f64, t2558: f64, t943: f64, t1897: f64, t28957: f64, t2936: f64, t10782: f64, t2580: f64, t7068: f64, t32112: f64, t954: f64, t13225: f64, t731: f64) -> (f64, f64, f64, f64, f64) {
    let t43127 = t943 * t11083 * t2558;
    let t43131 = 0.23071578690426672851e-1_f64 * t1897 * t2936 * t28957;
    let t43134 = t1897 * t2580 * t10782 * t7068;
    let t43137 = t1897 * t954 * t32112;
    let t43139 = t731 * t13225;
    (t43127, t43131, t43134, t43137, t43139)
}
