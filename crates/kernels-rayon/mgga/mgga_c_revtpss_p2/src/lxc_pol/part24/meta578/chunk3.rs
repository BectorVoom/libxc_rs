//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1784/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1784(t17396: f64, t1791: f64, t21014: f64, t24729: f64, t24731: f64, t24734: f64, t24741: f64, t24753: f64, t24840: f64, t3671: f64, t371: f64, t372: f64, t3720: f64, t482: f64, t5331: f64, t5340: f64, t57710: f64, t59411: f64, t70112: f64, t70133: f64, t82859: f64, t83114: f64, t83158: f64, t89808: f64) -> f64 {
    let t90998 = -0.34299214494455789578e-2_f64 * t83158 - 0.20325460441158986416e-2_f64 * t70112 - 0.31758531939310916276e-3_f64 * t70133 - 0.13719685797782315831e-1_f64 * t57710 * t24840 + 0.51448821741683684368e-2_f64 * t59411 * t24741 + 0.13719685797782315831e-1_f64 * t17396 * t24753 + 0.17149607247227894789e-2_f64 * t5340 * t3720 * t82859 * t24729 - 0.85748036236139473944e-3_f64 * t5331 * t3720 * t82859 * t24734 - 0.27439371595564631662e-1_f64 * t21014 * t24731 - 0.86891343385954666928e-1_f64 * t83114 * t1791 + 0.12862205435420921092e-2_f64 * t3671 * t371 * t372 * t482 * t89808;
    t90998
}
