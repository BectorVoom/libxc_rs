//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 3 (v3rho3_1) CSE chunk 1166/1255 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part3_v3rho3_1_chunk1166(t15035: f64, t15238: f64, t491: f64, t1246: f64, t15026: f64, t3623: f64, t11889: f64, t3507: f64, t1755: f64, t15018: f64, t3612: f64, t5075: f64, t5079: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15239 = t15035 + t15238;
    let t15240 = t491 * t15239;
    let t15241 = t15240 * t1246;
    let t15245 = t15026 * t3623;
    let t15247 = t11889 * t3507;
    let t15248 = t1755 * t15247;
    let t15253 = t15018 * t3612;
    let t15257 = t5075 * t5079;
    (t15239, t15241, t15245, t15248, t15253, t15257)
}
