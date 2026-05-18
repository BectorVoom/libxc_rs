//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1132/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1132<F: Float>(t13791: F, t14326: F, t10416: F, t1045: F, t14302: F, t3255: F, t4576: F, t4582: F, t4568: F, t13462: F, t4565: F, t10386: F, t347: F) -> (F, F, F, F, F, F, F) {
    let t14327 = t14326 * t13791;
    let t14331 = t10416 * t14302 * t1045;
    let t14339 = F::new(0.8760572888888888889e-3) * t3255 * t4576;
    let t14341 = F::new(0.17521145777777777778e-2) * t3255 * t4582;
    let t14343 = F::new(0.14600954814814814815e-2) * t3255 * t4568;
    let t14344 = t4565 * t13462;
    let t14347 = t10386 * t347;
    (t14327, t14331, t14339, t14341, t14343, t14344, t14347)
}
