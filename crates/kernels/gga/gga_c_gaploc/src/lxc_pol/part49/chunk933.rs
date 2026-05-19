//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 933/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk933<F: Float>(t3153: F, t8072: F, t10497: F, t2437: F, t2441: F, t34131: F, t895: F, t41838: F, t493: F, t1441: F, t590: F, t4130: F, t4781: F) -> (F, F, F, F, F, F) {
    let t42029 = F::cast_from(0.35750489951850426669e0_f64) * t3153 * t8072;
    let t42030 = t2437 * t10497;
    let t42032 = t2441 * t10497;
    let t42034 = t895 * t34131;
    let t42036 = t493 * t41838;
    let t42038 = t1441 * t42036 * t590;
    let t42042 = t4781 * t4130 * t41838 * t590;
    (t42029, t42030, t42032, t42034, t42038, t42042)
}
