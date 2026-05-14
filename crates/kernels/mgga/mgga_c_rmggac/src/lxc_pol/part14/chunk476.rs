//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 476/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk476<F: Float>(t1413: F, t385: F, t1425: F, t1529: F, t1532: F, t4155: F, t4163: F, t4173: F, t4182: F, t4214: F, t4220: F, t4336: F, t4338: F, t4586: F, t5385: F, t5388: F, t5389: F, t5392: F, t5393: F, t5394: F, t5395: F, t5402: F, t5403: F, t5405: F, t5407: F) -> (F, F) {
    let t5409 = 8.0 * t385 * t1413;
    let t5410 = t5385 - 0.62182e-1 * t1529 * t1532 - t5388 - 0.93273e-1 * t4182 * t5389 - t4155 - t4163 - t5392 - t5393 - t5394 + 0.186546e0 * t5395 * t4586 + 0.93273e-1 * t1425 * t4173 - t5402 + t5403 + t4336 - t4338 + t4214 - t4220 - t5405 + t5407 - t5409;
    (t5409, t5410)
}
