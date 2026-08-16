//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 574/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk574(t808: f64, t9688: f64, t568: f64, t836: f64, t1445: f64, t9735: f64, t1457: f64, t9730: f64, t3266: f64, t773: f64, t1: f64, t3209: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10068 = t808 * t9688;
    let t10069 = t568 * t10068;
    let t10076 = t836 * t9688;
    let t10077 = t568 * t10076;
    let t10080 = t1445 * t9735;
    let t10083 = t1457 * t9735;
    let t10086 = t1457 * t9730;
    let t10089 = t773 * t3266;
    let t10094 = t3209 * t1;
    (t10069, t10077, t10080, t10083, t10086, t10089, t10094)
}
