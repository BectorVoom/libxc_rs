//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1015/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1015<F: Float>(t3255: F, t4576: F, t4582: F, t4568: F, t13462: F, t4565: F, t10386: F, t347: F, t13467: F, t13516: F, t1662: F, t2952: F, t3269: F, t4621: F, t934: F, t3096: F, t3274: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14339 = 0.8760572888888888889e-3 * t3255 * t4576;
    let t14341 = 0.17521145777777777778e-2 * t3255 * t4582;
    let t14343 = 0.14600954814814814815e-2 * t3255 * t4568;
    let t14344 = t4565 * t13462;
    let t14347 = t10386 * t347;
    let t14348 = t14347 * t13467;
    let t14351 = t4565 * t13516;
    let t14355 = t3269 * t1662 * t2952;
    let t14359 = t3269 * t4621 * t934;
    let t14363 = t3274 * t1662 * t3096;
    (t14339, t14341, t14343, t14344, t14348, t14351, t14355, t14359, t14363)
}
