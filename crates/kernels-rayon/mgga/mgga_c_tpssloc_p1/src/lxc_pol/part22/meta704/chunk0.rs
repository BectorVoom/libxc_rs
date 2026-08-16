//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2292/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2292(t1222: f64, t18982: f64, t13969: f64, t18947: f64, t3506: f64, t11719: f64, t18302: f64, t1174: f64, t18225: f64, t3431: f64, t18221: f64, t15522: f64, t4889: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t66410 = t18982 * t1222;
    let t66413 = t3506 * t13969 * t18947;
    let t66437 = t11719 * t13969 * t18302;
    let t66449 = t1174 * t3431 * t18225;
    let t66452 = t1174 * t3431 * t18221;
    let t66458 = t4889 * t15522;
    (t66410, t66413, t66437, t66449, t66452, t66458)
}
