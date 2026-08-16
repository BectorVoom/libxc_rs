//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 190/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk190<F: Float>(t1105: F, t135: F, t150: F, t1091: F, t245: F, t410: F, t171: F, t977: F, t417: F, t978: F, t971: F, t1038: F) -> (F, F, F, F, F, F) {
    let t1106 = F::cast_from(1.0_f64) / t1105;
    let t1107 = t135 * t1106;
    let t1108 = t150 * t150;
    let t1109 = F::cast_from(1.0_f64) / t1108;
    let t1110 = t1091 * t1109;
    let t1112 = F::cast_from(0.16081979498692535067e2_f64) * t1107 * t1110;
    let t1116 = t245 * t410;
    let t1120 = t171 * t977;
    let t1121 = t978 * t417;
    let t1124 = t971 * t417;
    let t1127 = t171 * t1038;
    (t1112, t1116, t1120, t1121, t1124, t1127)
}
