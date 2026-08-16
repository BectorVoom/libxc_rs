//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1170/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1170(t1659: f64, t9895: f64, t3205: f64, t4519: f64, t10038: f64, t10042: f64, t10232: f64, t1268: f64, t1270: f64, t12907: f64, t12909: f64, t12911: f64, t12913: f64, t12915: f64, t12918: f64, t12919: f64, t12921: f64, t12923: f64, t12924: f64, t13111: f64, t1625: f64, t198: f64, t3183: f64, t3184: f64, t3202: f64, t4397: f64, t4524: f64, t509: f64, t7979: f64, t7988: f64, t7992: f64) -> (f64, f64) {
    let t13115 = t1659 * t9895;
    let t13119 = t4519 * t3205;
    let t13129 = t1270 * t13111 * t198 * t509 + 3.0_f64 * t10232 * t1625 * t3183 - 2.0_f64 * t1268 * t13119 * t4524 + 2.0_f64 * t13115 * t3202 * t4524 + 6.0_f64 * t3183 * t3184 * t4397 - t10038 - t10042 - t12907 + t12909 - t12911 + t12913 - t12915 + t12918 + t12919 - t12921 - t12923 + t12924 + t7979 + t7988 + t7992;
    (t13119, t13129)
}
