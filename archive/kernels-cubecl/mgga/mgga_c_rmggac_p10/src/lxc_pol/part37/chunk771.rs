//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 771/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk771<F: Float>(t2046: F, t2050: F, t2408: F, t31: F, t2039: F, t2406: F, t270: F, t638: F, t34738: F, t656: F, t8982: F, t36471: F, t8985: F) -> (F, F, F, F) {
    let t73953 = t2046 * t2050 * t2408 * t31;
    let t73957 = t638 * t2039 * t2406 * t270;
    let t73960 = t34738 * t656 * t8982;
    let t73963 = t36471 * t656 * t8985;
    (t73953, t73957, t73960, t73963)
}
