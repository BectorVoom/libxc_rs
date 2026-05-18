//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 890/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk890<F: Float>(t2305: F, t35658: F, t7255: F, t8497: F, t35654: F, t1986: F, t5160: F, t675: F, t2191: F, t8587: F, t26857: F, t7518: F) -> (F, F, F, F, F, F) {
    let t39401 = t35658 * t2305;
    let t39403 = t7255 * t8497;
    let t39405 = t35654 * t2305;
    let t39406 = F::new(0.19863479950205658386e-4) * t39405;
    let t39418 = t675 * t1986 * t5160;
    let t39420 = t2191 * t8587;
    let t39423 = t26857 * t7518;
    (t39401, t39403, t39406, t39418, t39420, t39423)
}
