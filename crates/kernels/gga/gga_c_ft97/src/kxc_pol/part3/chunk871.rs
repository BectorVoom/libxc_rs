//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 871/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk871<F: Float>(t17780: F, t4206: F, t19030: F, t2771: F, t17727: F, t17732: F, t4199: F, t19267: F, t10613: F, t19271: F, t17766: F, t1775: F, t5349: F, t5352: F, t5343: F, t2: F, t5299: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t19669 = t4206 * t17780;
    let t19672 = t2771 * t19030;
    let t19675 = t4206 * t17727;
    let t19678 = t4199 * t17732;
    let t19681 = t2771 * t19267;
    let t19684 = t10613 * t19271;
    let t19687 = t4199 * t17766;
    let t19691 = t1775 * t5349;
    let t19693 = t1775 * t5352;
    let t19695 = t1775 * t5343;
    let t19697 = t2 * t5299;
    (t19669, t19672, t19675, t19678, t19681, t19684, t19687, t19691, t19693, t19695, t19697)
}
