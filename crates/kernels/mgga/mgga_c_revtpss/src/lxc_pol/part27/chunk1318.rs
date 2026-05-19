//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1318/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1318<F: Float>(t12963: F, t7613: F, t12975: F, t2138: F, t12984: F, t12966: F, t12851: F, t2134: F, t12282: F, t12287: F, t1238: F, t12812: F, t12872: F, t12889: F, t12945: F, t12972: F, t13076: F, t26827: F, t26873: F, t29047: F, t29048: F, t29054: F, t29097: F, t3591: F, t3663: F, t3674: F, t3714: F, t484: F, t7618: F, t7624: F, t97250: F, t97261: F, t97267: F, t97269: F, t97272: F, t97279: F) -> F {
    let t97281 = t7613 * t12963;
    let t97283 = t12975 * t2138;
    let t97288 = t7613 * t12984;
    let t97292 = t12966 * t2138;
    let t97296 = F::new(5.0) / F::new(1296.0) * t2134 * t12851;
    let t97297 = F::cast_from(0.17149607247227894789e-2_f64) * t97250 * t3714 + F::cast_from(0.25724410870841842183e-2_f64) * t29097 * t12872 - t29047 * t29048 * t12287 / F::new(48.0) + t29047 * t29054 * t12282 / F::new(72.0) + F::cast_from(0.12862205435420921092e-2_f64) * t97261 * t12812 + F::cast_from(0.42874018118069736972e-3_f64) * t12889 * t2138 * t484 - F::cast_from(0.28582678745379824648e-3_f64) * t97267 + F::cast_from(0.85748036236139473944e-3_f64) * t97269 + t97272 + F::cast_from(0.14291339372689912324e-2_f64) * t7624 * t12945 + F::cast_from(0.12862205435420921092e-2_f64) * t26873 * t3591 + F::cast_from(0.42874018118069736972e-3_f64) * t7618 * t13076 + F::cast_from(0.17149607247227894789e-2_f64) * t97279 - F::cast_from(0.85748036236139473944e-3_f64) * t97281 - F::cast_from(0.12862205435420921092e-2_f64) * t97283 * t1238 - F::cast_from(0.12862205435420921092e-2_f64) * t26827 * t3663 + F::cast_from(0.28582678745379824648e-3_f64) * t97288 - F::cast_from(0.42874018118069736972e-3_f64) * t7613 * t12972 + F::cast_from(0.25724410870841842183e-2_f64) * t97292 * t3674 + t97296;
    t97297
}
