//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 683/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk683<F: Float>(t8369: F, t8371: F, t8374: F, t8377: F, t8382: F, t8385: F, t8388: F, t8391: F, t8397: F, t8401: F, t8403: F, t1600: F, t2958: F, t1573: F, t2932: F, t2938: F) -> (F, F, F) {
    let t8405 = 0.3475929712541504153e-2 * t8369 + 0.3475929712541504153e-2 * t8371 + 0.20855578275249024918e-2 * t8374 - 0.43449121406768801912e-5 * t8377 + 0.257508459537449766e-6 * t8382 - 0.772525378612349298e-5 * t8385 - 0.10427789137624512459e-2 * t8388 - 0.6951859425083008306e-4 * t8391 + 0.73256006569213709438e-5 * t8397 + 0.43449121406768801912e-4 * t8401 - 0.27517776890953574544e-3 * t8403;
    let t8406 = t1600 * t2958;
    let t8408 = t2932 * t1573;
    let t8409 = t8408 * t2938;
    (t8405, t8406, t8409)
}
