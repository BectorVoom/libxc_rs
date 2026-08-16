//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3199/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3199(t1219: f64, t24551: f64, t21254: f64, t5373: f64, t20858: f64, t21200: f64, t21203: f64, t44931: f64, t59144: f64, t59411: f64, t71470: f64, t71476: f64, t71490: f64, t71539: f64, t71541: f64) -> f64 {
    let t84029 = t24551 * t1219;
    let t84032 = t5373 * t21254;
    let t84036 = 0.57165357490759649295e-3_f64 * t71470 + 0.45732285992607719436e-2_f64 * t71476 + 0.12862205435420921092e-2_f64 * t59411 * t20858 - 0.25724410870841842183e-2_f64 * t71490 - 0.13719685797782315831e-1_f64 * t21203 * t21200 - 5.0_f64 / 3888.0_f64 * t44931 - 77.0_f64 / 486.0_f64 * t84029 - 5.0_f64 / 162.0_f64 * t59144 + t84032 / 54.0_f64 + 0.28582678745379824648e-2_f64 * t71539 + 0.57165357490759649295e-3_f64 * t71541;
    t84036
}
