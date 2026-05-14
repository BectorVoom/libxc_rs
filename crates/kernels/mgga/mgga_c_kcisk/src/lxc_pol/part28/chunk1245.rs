//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1245/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1245<F: Float>(t35308: F, t35336: F, t1908: F, t2815: F, t9291: F, t35283: F, t35285: F, t35287: F, t35289: F, t35291: F, t35293: F, t35295: F, t35297: F, t35299: F, t35302: F, t35304: F, t35306: F) -> (F, F, F, F) {
    let t35337 = t35308 + t35336;
    let t35338 = t1908 * t35337;
    let t35344 = t2815 * t9291;
    let t35360 = 0.4046875e-1 * t35283 + 0.21583333333333333334e0 * t35285 - 0.53958333333333333334e-1 * t35287 - 0.21583333333333333334e0 * t35289 + 0.53958333333333333334e-1 * t35291 + 0.5e0 * t35293 - 0.125e0 * t35295 - 0.89930555555555555557e-2 * t35297 - 0.26979166666666666667e-1 * t35299 + 0.1875e0 * t35302 - 0.4046875e-1 * t35304 + 0.91666666666666666667e0 * t35306;
    (t35337, t35338, t35344, t35360)
}
