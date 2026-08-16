//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1788/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1788(t6587: f64, t6622: f64, t1042: f64, t12787: f64, t1715: f64, t20795: f64, t20809: f64, t3711: f64, t44190: f64, t5340: f64, t57471: f64, t5819: f64, t6429: f64, t6640: f64, t6690: f64, t70758: f64, t71275: f64, t71513: f64, t82816: f64, t83504: f64, t83539: f64, t83558: f64, t83580: f64) -> (f64, f64) {
    let t91199 = t6587 * t6622;
    let t91228 = -0.2540682555144873302e-3_f64 * t57471 - t83504 / 36.0_f64 - 0.86891343385954666928e-1_f64 * t71513 * t6690 + 0.57165357490759649296e-3_f64 * t3711 * t1042 * t82816 * t1715 + 0.11433071498151929859e-2_f64 * t83539 + 0.85748036236139473944e-3_f64 * t3711 * t1042 * t20809 * t6429 - 0.16937883700965822014e-2_f64 * t83558 + 0.57165357490759649296e-3_f64 * t70758 + 0.17149607247227894789e-2_f64 * t83580 + 0.18292914397043087775e-1_f64 * t71275 * t6640 + 0.28582678745379824648e-2_f64 * t5340 * t12787 * t20795 * t44190 * t5819;
    (t91199, t91228)
}
