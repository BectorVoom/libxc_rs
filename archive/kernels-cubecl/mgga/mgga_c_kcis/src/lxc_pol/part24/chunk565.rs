//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 565/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk565<F: Float>(t1220: F, t5233: F, t1830: F, t3577: F, t1219: F, t1684: F, t962: F, t1835: F, t969: F, t4758: F, t971: F, t1692: F, t3034: F) -> (F, F, F, F, F, F, F) {
    let t5234 = t5233 * t1220;
    let t5237 = t1830 * t3577;
    let t5238 = t5237 * t1219;
    let t5242 = t1684 * t962;
    let t5247 = t1835 * t969;
    let t5250 = t4758 * t971;
    let t5253 = t1692 * t3034;
    (t5234, t5237, t5238, t5242, t5247, t5250, t5253)
}
