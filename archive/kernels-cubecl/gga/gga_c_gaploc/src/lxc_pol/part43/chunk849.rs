//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 849/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk849<F: Float>(t20535: F, t34688: F, t9537: F, t26796: F, t9282: F, t20671: F, t31037: F, t35101: F, t12925: F, t4614: F, t574: F, t3133: F, t4752: F, t8352: F) -> (F, F, F, F, F) {
    let t42066 = t20535 * t34688 * t9537;
    let t42067 = F::cast_from(0.11502877786176224903e1_f64) * t42066;
    let t42069 = F::cast_from(0.47667319935800568892e0_f64) * t26796 * t9282;
    let t42071 = t31037 * t20671 * t35101;
    let t42072 = F::cast_from(0.25561950635947166451e0_f64) * t42071;
    let t42081 = F::cast_from(0.61348681526273199483e1_f64) * t574 * t4614 * t12925;
    let t42092 = F::cast_from(0.28600391961480341335e1_f64) * t8352 * t4752 * t3133;
    (t42067, t42069, t42072, t42081, t42092)
}
