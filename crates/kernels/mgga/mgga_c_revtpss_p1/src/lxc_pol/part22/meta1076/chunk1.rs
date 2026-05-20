//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3857/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3857<F: Float>(t47099: F, t22212: F, t2626: F, t1320: F, t22195: F, t47101: F, t48313: F, t47110: F, t47113: F, t47119: F, t47125: F, t40067: F, t40072: F, t47098: F, t47109: F, t47116: F, t47118: F, t47122: F, t47124: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t74129 = F::cast_from(0.5848223622634646207e0_f64) * t47099;
    let t74130 = t22212 * t2626;
    let t74131 = F::cast_from(0.11696447245269292414e1_f64) * t74130;
    let t74132 = t1320 * t22195;
    let t74133 = F::new(8.0) * t74132;
    let t74134 = F::new(64.0) * t47101;
    let t74135 = F::cast_from(0.43374325201206959368e-1_f64) * t48313;
    let t74136 = F::cast_from(0.70178683471615754484e1_f64) * t47110;
    let t74137 = F::new(2.0) * t47113;
    let t74138 = F::cast_from(0.65061487801810439052e-1_f64) * t47119;
    let t74139 = F::cast_from(0.96319466275353142156e0_f64) * t47125;
    let t74140 = -t47098 - t74129 + t74131 - t74133 + t74134 - t74135 + t40067 - t40072 - t47109 - t74136 + t74137 + t47116 - t47118 - t74138 + t47122 + t47124 + t74139;
    (t74129, t74131, t74133, t74134, t74135, t74136, t74137, t74138, t74139, t74140)
}
