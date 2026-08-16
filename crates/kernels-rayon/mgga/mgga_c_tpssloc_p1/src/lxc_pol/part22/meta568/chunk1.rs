//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2075/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2075(t10383: f64, t964: f64, t10868: f64, t820: f64, t1015: f64, t10472: f64, t42559: f64, t204: f64, t376: f64, t1020: f64, t1023: f64, t248: f64) -> (f64, f64, f64, f64, f64) {
    let t43157 = t964 * t10383;
    let t43198 = t820 * t10868;
    let t43211 = t10472 * t1015 * t42559;
    let t43216 = t204 * t376;
    let t43219 = t1020 * t248 * t43216 * t1023;
    (t43157, t43198, t43211, t43216, t43219)
}
