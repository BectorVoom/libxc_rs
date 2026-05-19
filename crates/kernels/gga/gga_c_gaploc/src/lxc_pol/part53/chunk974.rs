//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 974/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk974<F: Float>(t39123: F, t959: F, t13847: F, t2684: F, t7354: F, t13872: F, t2194: F, t47311: F, t568: F, t808: F, t813: F, t13883: F, t1589: F, t797: F) -> (F, F, F, F, F) {
    let t47381 = t39123 * t959;
    let t47389 = t2684 * t7354 * t13847;
    let t47408 = F::cast_from(0.23005755572352449806e1_f64) * t2194 * t13872;
    let t47412 = F::cast_from(0.23005755572352449806e1_f64) * t813 * t568 * t808 * t47311;
    let t47415 = F::cast_from(0.23833659967900284446e0_f64) * t797 * t1589 * t13883;
    (t47381, t47389, t47408, t47412, t47415)
}
