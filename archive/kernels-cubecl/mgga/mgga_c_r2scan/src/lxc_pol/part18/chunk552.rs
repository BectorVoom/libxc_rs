//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 552/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk552<F: Float>(t3034: F, t86: F, t88: F, t41: F, t1387: F, t1413: F, t1418: F, t1421: F, t2896: F, t2897: F, t2997: F, t2998: F, t3020: F) -> (F, F, F, F) {
    let t3035 = t3034 * t86;
    let t3036 = F::cast_from(0.19751673498613801407e-1_f64) * t3035;
    let t3037 = t3034 * t88;
    let t3038 = t41 * t3037;
    let t3039 = -t3020 + t2896 - t2897 - t2998 + t3036 + t3038 - t2997 - t1387 - t1413 + t1418 + t1421;
    (t3036, t3037, t3038, t3039)
}
