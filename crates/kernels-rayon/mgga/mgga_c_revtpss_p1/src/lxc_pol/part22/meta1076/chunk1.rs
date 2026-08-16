//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3857/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3857(t47099: f64, t22212: f64, t2626: f64, t1320: f64, t22195: f64, t47101: f64, t48313: f64, t47110: f64, t47113: f64, t47119: f64, t47125: f64, t40067: f64, t40072: f64, t47098: f64, t47109: f64, t47116: f64, t47118: f64, t47122: f64, t47124: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t74129 = 0.5848223622634646207e0_f64 * t47099;
    let t74130 = t22212 * t2626;
    let t74131 = 0.11696447245269292414e1_f64 * t74130;
    let t74132 = t1320 * t22195;
    let t74133 = 8.0_f64 * t74132;
    let t74134 = 64.0_f64 * t47101;
    let t74135 = 0.43374325201206959368e-1_f64 * t48313;
    let t74136 = 0.70178683471615754484e1_f64 * t47110;
    let t74137 = 2.0_f64 * t47113;
    let t74138 = 0.65061487801810439052e-1_f64 * t47119;
    let t74139 = 0.96319466275353142156e0_f64 * t47125;
    let t74140 = -t47098 - t74129 + t74131 - t74133 + t74134 - t74135 + t40067 - t40072 - t47109 - t74136 + t74137 + t47116 - t47118 - t74138 + t47122 + t47124 + t74139;
    (t74129, t74131, t74133, t74134, t74135, t74136, t74137, t74138, t74139, t74140)
}
