//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1239/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1239<F: Float>(t12079: F, t31299: F, t31346: F, t35054: F, t35075: F, t35090: F, t35094: F, t35097: F, t35100: F, t35104: F, t35110: F, t35113: F, t35116: F, t35120: F, t38731: F, t4372: F, t6710: F, t6711: F, t6716: F, t6717: F) -> (F,) {
    let t38824 = t35054 - 0.23005755572352449806e2 * t6710 * t6711 * t38731 + 0.13803453343411469884e2 * t6716 * t6717 * t38731 + t31299 - t35075 + 0.92686455430723328401e-1 * t12079 * t4372 - t35090 - t35094 - t35097 + t35100 - t35104 + t35110 + t35113 + t35116 - t35120 - 0.76685851907841499354e0 * t31346;
    (t38824,)
}
