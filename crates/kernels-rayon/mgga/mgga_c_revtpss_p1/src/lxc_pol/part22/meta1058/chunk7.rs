//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3760/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3760(t20913: f64, t3172: f64, t3711: f64, t3147: f64, t6593: f64, t3594: f64, t3597: f64, t1244: f64, t1042: f64, t1222: f64, t17500: f64, t17541: f64, t17569: f64, t17584: f64, t17700: f64, t20982: f64, t20986: f64, t21102: f64, t3591: f64, t3606: f64, t3613: f64, t3647: f64, t5056: f64, t5299: f64, t5308: f64, t5391: f64, t57053: f64, t68299: f64, t68303: f64) -> f64 {
    let t71687 = t3711 * t3172 * t20913;
    let t71691 = t6593 * t3147;
    let t71693 = t3594 * t3597 * t71691;
    let t71699 = t3594 * t1244 * t71691;
    let t71704 = -t1222 * t5308 * t68299 / 144.0_f64 - t1222 * t5308 * t68303 / 48.0_f64 + 0.57165357490759649296e-3_f64 * t3711 * t1042 * t17500 * t5056 - 0.11433071498151929859e-2_f64 * t3647 * t20982 - 0.17149607247227894789e-2_f64 * t3647 * t20986 + 0.57165357490759649296e-3_f64 * t57053 * t5299 - 0.5081365110289746604e-2_f64 * t5391 * t17700 + 0.3811023832717309953e-3_f64 * t71687 + 0.72409452821628889107e-2_f64 * t21102 * t3591 + 0.14481890564325777821e-1_f64 * t71693 * t3606 + 0.28582678745379824648e-3_f64 * t17569 * t17541 - 0.72409452821628889107e-2_f64 * t71699 * t3613 + 0.28582678745379824648e-3_f64 * t17569 * t17584;
    t71704
}
