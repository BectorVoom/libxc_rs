//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1743/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1743(t6622: f64, t482: f64, t1042: f64, t1261: f64, t17569: f64, t1774: f64, t21275: f64, t22671: f64, t24605: f64, t24649: f64, t24726: f64, t24836: f64, t3711: f64, t5296: f64, t5302: f64, t5381: f64, t59162: f64, t6635: f64, t70319: f64, t82595: f64, t82603: f64, t82656: f64, t82678: f64, t88916: f64) -> (f64, f64, f64) {
    let t90080 = t6622 * t6622;
    let t90081 = t482 * t90080;
    let t90116 = -0.12862205435420921092e-2_f64 * t70319 * t6635 - 0.34299214494455789578e-2_f64 * t21275 * t24605 + 0.95275595817932748828e-3_f64 * t1261 * t1042 * t5302 * t88916 + 0.17149607247227894789e-2_f64 * t17569 * t24649 + 0.57165357490759649296e-3_f64 * t3711 * t1042 * t5296 * t22671 * t1774 + 0.1219527626469539185e-1_f64 * t82595 - 0.38110238327173099531e-3_f64 * t82603 - 0.34299214494455789577e-2_f64 * t5381 * t24726 + 0.19055119163586549765e-2_f64 * t82656 - 0.51448821741683684368e-2_f64 * t59162 * t24836 - 0.34299214494455789578e-2_f64 * t82678;
    (t90080, t90081, t90116)
}
