//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 801/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk801<F: Float>(t1494: F, t209: F, t26: F, t14163: F, t2067: F, t3369: F, t3352: F, t495: F, t515: F, t7230: F, t8975: F, t15405: F, t7255: F) -> (F, F, F) {
    let t74411 = t26 * t1494 * t209;
    let t74414 = t14163 * t3369 * t2067 * t74411;
    let t74419 = t7230 * t3352 * t515 * t8975 * t495;
    let t74421 = t7255 * t15405;
    (t74414, t74419, t74421)
}
