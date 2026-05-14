//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 784/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk784<F: Float>(t2019: F, t2020: F, t8862: F, t34944: F, t5268: F, t656: F, t236: F, t3351: F, t5207: F, t9188: F, t7244: F, t8497: F, t7914: F, t8571: F, t1243: F, t551: F, t7248: F) -> (F, F, F, F, F, F) {
    let t39255 = t2019 * t2020 * t8862;
    let t39256 = 0.30487649791575028314e-3 * t39255;
    let t39258 = t34944 * t656 * t5268;
    let t39262 = t3351 * t9188 * t236 * t5207;
    let t39264 = t7244 * t8497;
    let t39265 = 0.19863479950205658386e-4 * t39264;
    let t39266 = t8571 * t7914;
    let t39271 = t3351 * t7248 * t236 * t551 * t1243;
    (t39256, t39258, t39262, t39265, t39266, t39271)
}
