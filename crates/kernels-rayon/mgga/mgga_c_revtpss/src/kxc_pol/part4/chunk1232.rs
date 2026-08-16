//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1232/1428 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1232(t15234: f64, t973: f64, t2962: f64, t4673: f64, t11452: f64, t1621: f64, t2944: f64, t4708: f64, t972: f64, t1634: f64, t3006: f64, t2988: f64, t4711: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15235 = t15234 * t973;
    let t15238 = t4673 * t2962;
    let t15241 = t1621 * t11452;
    let t15242 = t15241 * t2944;
    let t15249 = t4708 * t972;
    let t15252 = t1634 * t3006;
    let t15255 = t4711 * t2988;
    (t15235, t15238, t15242, t15249, t15252, t15255)
}
