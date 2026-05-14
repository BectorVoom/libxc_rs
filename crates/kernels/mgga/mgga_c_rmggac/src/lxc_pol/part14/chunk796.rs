//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 796/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk796<F: Float>(t7720: F, t8587: F, t34847: F, t9206: F, t1001: F, t236: F, t615: F, t7230: F, t9210: F, t1166: F, t1979: F, t1982: F, t2313: F, t7501: F, t8562: F, t2139: F, t27: F, t4928: F, t649: F) -> (F, F, F, F, F, F) {
    let t39463 = t7720 * t8587;
    let t39465 = t34847 * t9206;
    let t39470 = t7230 * t9210 * t236 * t615 * t1001;
    let t39474 = t2313 * t1166 * t1979 * t1982;
    let t39482 = t7501 * t8562;
    let t39486 = t2139 * t27 * t649 * t4928;
    (t39463, t39465, t39470, t39474, t39482, t39486)
}
