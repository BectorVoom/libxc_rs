//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1743/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1743<F: Float>(t6622: F, t482: F, t1042: F, t1261: F, t17569: F, t1774: F, t21275: F, t22671: F, t24605: F, t24649: F, t24726: F, t24836: F, t3711: F, t5296: F, t5302: F, t5381: F, t59162: F, t6635: F, t70319: F, t82595: F, t82603: F, t82656: F, t82678: F, t88916: F) -> (F, F, F) {
    let t90080 = t6622 * t6622;
    let t90081 = t482 * t90080;
    let t90116 = -F::cast_from(0.12862205435420921092e-2_f64) * t70319 * t6635 - F::cast_from(0.34299214494455789578e-2_f64) * t21275 * t24605 + F::cast_from(0.95275595817932748828e-3_f64) * t1261 * t1042 * t5302 * t88916 + F::cast_from(0.17149607247227894789e-2_f64) * t17569 * t24649 + F::cast_from(0.57165357490759649296e-3_f64) * t3711 * t1042 * t5296 * t22671 * t1774 + F::cast_from(0.1219527626469539185e-1_f64) * t82595 - F::cast_from(0.38110238327173099531e-3_f64) * t82603 - F::cast_from(0.34299214494455789577e-2_f64) * t5381 * t24726 + F::cast_from(0.19055119163586549765e-2_f64) * t82656 - F::cast_from(0.51448821741683684368e-2_f64) * t59162 * t24836 - F::cast_from(0.34299214494455789578e-2_f64) * t82678;
    (t90080, t90081, t90116)
}
