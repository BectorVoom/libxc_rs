//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 770/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk770<F: Float>(t1971: F, t27177: F, t3351: F, t7190: F, t615: F, t7230: F, t875: F, t876: F, t16156: F, t8812: F, t2320: F, t35265: F, t1175: F, t236: F, t3352: F, t551: F, t8517: F) -> (F, F, F, F, F) {
    let t38991 = t3351 * t1971 * t7190 * t27177;
    let t38996 = t7230 * t1971 * t875 * t615 * t876;
    let t38998 = t16156 * t8812;
    let t39003 = t35265 * t2320;
    let t39009 = t8517 * t3352 * t236 * t551 * t1175;
    (t38991, t38996, t38998, t39003, t39009)
}
