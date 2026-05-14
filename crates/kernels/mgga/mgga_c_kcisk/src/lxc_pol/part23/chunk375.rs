//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 375/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk375<F: Float>(t1484: F, t1490: F, t1494: F, t1498: F, t1502: F, t1507: F, t1514: F, t1518: F) -> (F,) {
    let t1620 = 0.9375e-1 * t1484 - 0.9375e-1 * t1490 - 0.25e0 * t1494 + 0.625e-1 * t1498 - 0.101171875e-1 * t1502 + 0.101171875e-1 * t1507 + 0.53958333333333333333e-1 * t1514 - 0.13489583333333333333e-1 * t1518;
    (t1620,)
}
