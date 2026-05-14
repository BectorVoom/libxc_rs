//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 779/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk779<F: Float>(t1965: F, t9085: F, t1969: F, t1973: F, t7259: F, t8577: F, t2305: F, t35658: F, t7255: F, t8497: F, t35654: F, t1986: F, t5160: F, t675: F, t2191: F, t8587: F) -> (F, F, F, F, F, F, F) {
    let t39392 = t9085 * t1965;
    let t39393 = t39392 * t1969;
    let t39394 = t39393 * t1973;
    let t39396 = t8577 * t7259;
    let t39401 = t35658 * t2305;
    let t39403 = t7255 * t8497;
    let t39405 = t35654 * t2305;
    let t39418 = t675 * t1986 * t5160;
    let t39420 = t2191 * t8587;
    (t39394, t39396, t39401, t39403, t39405, t39418, t39420)
}
