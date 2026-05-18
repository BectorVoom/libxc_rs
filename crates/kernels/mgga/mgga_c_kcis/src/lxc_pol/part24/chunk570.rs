//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 570/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk570<F: Float>(t4567: F, t5302: F, t1262: F, t1662: F, t3515: F, t421: F, t993: F) -> (F, F, F, F) {
    let t5303 = t5302 * t4567;
    let t5306 = t1662 * t1262;
    let t5307 = t3515 * t5306;
    let t5310 = t993 * t421;
    (t5303, t5306, t5307, t5310)
}
