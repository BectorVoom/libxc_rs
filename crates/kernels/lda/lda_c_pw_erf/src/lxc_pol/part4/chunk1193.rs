//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1193/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1193<F: Float>(t13258: F, t13261: F, t13298: F, t13301: F, t13303: F, t13306: F, t13309: F, t13318: F, t13351: F, t13358: F, t13366: F, t13375: F, t10278: F, t10286: F, t13377: F, t13379: F, t13381: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t17608 = 16.0 / 135.0 * t13258;
    let t17609 = 16.0 / 81.0 * t13261;
    let t17610 = 64.0 / 135.0 * t13298;
    let t17611 = 32.0 / 135.0 * t13301;
    let t17612 = 64.0 / 135.0 * t13303;
    let t17613 = 32.0 / 135.0 * t13306;
    let t17614 = 128.0 / 135.0 * t13309;
    let t17615 = 32.0 / 135.0 * t13318;
    let t17616 = 32.0 / 135.0 * t13351;
    let t17617 = 64.0 / 405.0 * t13358;
    let t17618 = 32.0 / 45.0 * t13366;
    let t17619 = 16.0 / 81.0 * t13375;
    let t17624 = -t17608 - t17609 + t17610 + t17611 + t17612 + t17613 + t17614 - t17615 + t17616 - t17617 + t17618 + t17619 + 8.0 * t13377 + 32.0 / 3.0 * t13379 + 8.0 / 3.0 * t13381 + 8.0 / 3.0 * t10278 + t10286;
    (t17608, t17609, t17610, t17611, t17612, t17613, t17614, t17615, t17616, t17617, t17618, t17619, t17624)
}
