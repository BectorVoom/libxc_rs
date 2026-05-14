//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1290/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1290<F: Float>(t12829: F, t1597: F, t32401: F, t33778: F, t13900: F, t9536: F, t9863: F, t12261: F, t2737: F, t9868: F, t32474: F, t33873: F, t33766: F, t9535: F, t113920: F, t113922: F) -> (F, F, F, F, F, F, F, F) {
    let t115284 = t1597 * t12829;
    let t115312 = 0.13402777777777777778e-2 * t33778 * t32401;
    let t115337 = t9536 * t13900 * t9863;
    let t115346 = t2737 * t12261 * t9868;
    let t115351 = 0.13402777777777777778e-2 * t32474 * t33873;
    let t115358 = t33766 * t9535;
    let t115374 = 0.30952962962962962962e-2 * t113920;
    let t115375 = 0.25794135802469135802e-2 * t113922;
    (t115284, t115312, t115337, t115346, t115351, t115358, t115374, t115375)
}
