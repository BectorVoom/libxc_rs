//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 866/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk866<F: Float>(t10009: F, t10012: F, t10016: F, t10020: F, t38300: F, t38304: F, t38305: F, t38306: F, t38307: F, t38308: F, t7709: F, t1970: F, t1971: F, t236: F, t6149: F) -> (F, F) {
    let t44568 = -t38300 + t10009 + t10012 + t7709 + t38304 + t10016 - t10020 + t38305 + t38306 + t38307 - t38308;
    let t44580 = t1970 * t1971 * t236 * t6149;
    (t44568, t44580)
}
