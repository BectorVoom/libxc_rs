//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1504/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1504(t11960: f64, t351: f64, t361: f64, t369: f64, t1041: f64, t11262: f64, t3135: f64, t1033: f64, t1036: f64, t1038: f64, t1042: f64, t1047: f64, t1065: f64, t1068: f64, t11173: f64, t11233: f64, t11281: f64, t11286: f64, t11656: f64, t11845: f64, t11983: f64, t2853: f64, t3059: f64, t3106: f64, t3127: f64, t3130: f64, t3181: f64, t42571: f64, t4837: f64, t906: f64) -> f64 {
    let t42576 = t351 * t361 * t11960 * t369;
    let t42580 = t1041 * t11262 * t3135;
    let t42584 = t1033 * t1036 * t11960 * t1038;
    let t42602 = 0.28582678745379824648e-2_f64 * t4837 * t1042 * t3181 * t3059 * t2853 + 0.18292914397043087775e-1_f64 * t42571 * t3130 - 0.14160070774007427203e0_f64 * t42576 * t1068 - 0.28582678745379824648e-3_f64 * t42580 - 0.21240106161011140804e0_f64 * t42584 * t1047 - 0.57165357490759649296e-3_f64 * t3127 * t1042 * t1065 * t11173 * t906 + 0.91464571985215438872e-2_f64 * t11656 * t11281 + 0.18292914397043087775e-1_f64 * t3106 * t11233 - 0.15244095330869239812e-1_f64 * t3106 * t11983 + 0.15244095330869239812e-1_f64 * t11656 * t11286 - 0.30488190661738479624e-2_f64 * t3106 * t11845;
    t42602
}
