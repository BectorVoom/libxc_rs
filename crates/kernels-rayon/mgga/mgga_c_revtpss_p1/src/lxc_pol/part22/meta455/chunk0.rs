//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2124/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2124(t3006: f64, t4711: f64, t11509: f64, t1633: f64, t2988: f64, t4670: f64, t953: f64, t1622: f64, t2962: f64, t2944: f64, t4673: f64, t2970: f64, t4669: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15263 = t4711 * t3006;
    let t15266 = t1633 * t11509;
    let t15267 = t15266 * t2988;
    let t15274 = t4670 * t953;
    let t15277 = t1622 * t2962;
    let t15280 = t4673 * t2944;
    let t15283 = t4669 * t2970;
    (t15263, t15266, t15267, t15274, t15277, t15280, t15283)
}
