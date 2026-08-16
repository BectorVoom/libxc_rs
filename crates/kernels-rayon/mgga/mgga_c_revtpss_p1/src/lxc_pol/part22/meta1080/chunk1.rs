//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3883/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3883(t2661: f64, t3992: f64, t48533: f64, t6869: f64, t14045: f64, t22096: f64, t21990: f64, t5608: f64, t9934: f64, t1353: f64, t13804: f64, t13805: f64, t1410: f64, t21969: f64, t22074: f64, t22079: f64, t3924: f64, t3934: f64, t3936: f64, t4012: f64, t47259: f64, t47262: f64, t5673: f64, t74579: f64, t74583: f64, t74585: f64, t74589: f64, t828: f64) -> f64 {
    let t74598 = t2661 * t3992 * t48533 * t6869;
    let t74602 = t2661 * t3992 * t14045 * t22096;
    let t74606 = t2661 * t9934 * t5608 * t21990;
    let t74616 = -0.18071592998981862716e-4_f64 * t47259 + 0.65057734796334705778e-3_f64 * t47262 + 0.11433071498151929859e-3_f64 * t74579 + 0.85748036236139473945e-4_f64 * t74583 - 0.56688979511669985553e-2_f64 * t74585 + 0.28582678745379824648e-3_f64 * t74589 + 0.85748036236139473944e-2_f64 * t1410 * t4012 * t828 * t21969 * t1353 - 0.11433071498151929859e-3_f64 * t74598 - 0.11433071498151929859e-3_f64 * t74602 - 0.57165357490759649296e-4_f64 * t74606 - 0.12862205435420921092e-2_f64 * t13804 * t5673 * t22079 * t13805 + 0.85748036236139473944e-3_f64 * t3934 * t3936 * t22074 * t3924;
    t74616
}
