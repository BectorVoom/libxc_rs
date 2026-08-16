//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1958/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1958(t11889: f64, t3507: f64, t1755: f64, t15018: f64, t3612: f64, t5075: f64, t5079: f64, t1706: f64, t3428: f64, t1184: f64, t460: f64, t4928: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15247 = t11889 * t3507;
    let t15248 = t1755 * t15247;
    let t15253 = t15018 * t3612;
    let t15257 = t5075 * t5079;
    let t15265 = t1706 * t3428;
    let t15268 = t4928 * t1184 * t460;
    (t15247, t15248, t15253, t15257, t15265, t15268)
}
