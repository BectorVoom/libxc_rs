//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1318/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1318(t12963: f64, t7613: f64, t12975: f64, t2138: f64, t12984: f64, t12966: f64, t12851: f64, t2134: f64, t12282: f64, t12287: f64, t1238: f64, t12812: f64, t12872: f64, t12889: f64, t12945: f64, t12972: f64, t13076: f64, t26827: f64, t26873: f64, t29047: f64, t29048: f64, t29054: f64, t29097: f64, t3591: f64, t3663: f64, t3674: f64, t3714: f64, t484: f64, t7618: f64, t7624: f64, t97250: f64, t97261: f64, t97267: f64, t97269: f64, t97272: f64, t97279: f64) -> f64 {
    let t97281 = t7613 * t12963;
    let t97283 = t12975 * t2138;
    let t97288 = t7613 * t12984;
    let t97292 = t12966 * t2138;
    let t97296 = 5.0_f64 / 1296.0_f64 * t2134 * t12851;
    let t97297 = 0.17149607247227894789e-2_f64 * t97250 * t3714 + 0.25724410870841842183e-2_f64 * t29097 * t12872 - t29047 * t29048 * t12287 / 48.0_f64 + t29047 * t29054 * t12282 / 72.0_f64 + 0.12862205435420921092e-2_f64 * t97261 * t12812 + 0.42874018118069736972e-3_f64 * t12889 * t2138 * t484 - 0.28582678745379824648e-3_f64 * t97267 + 0.85748036236139473944e-3_f64 * t97269 + t97272 + 0.14291339372689912324e-2_f64 * t7624 * t12945 + 0.12862205435420921092e-2_f64 * t26873 * t3591 + 0.42874018118069736972e-3_f64 * t7618 * t13076 + 0.17149607247227894789e-2_f64 * t97279 - 0.85748036236139473944e-3_f64 * t97281 - 0.12862205435420921092e-2_f64 * t97283 * t1238 - 0.12862205435420921092e-2_f64 * t26827 * t3663 + 0.28582678745379824648e-3_f64 * t97288 - 0.42874018118069736972e-3_f64 * t7613 * t12972 + 0.25724410870841842183e-2_f64 * t97292 * t3674 + t97296;
    t97297
}
