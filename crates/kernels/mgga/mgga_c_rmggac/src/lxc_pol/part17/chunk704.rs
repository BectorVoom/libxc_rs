//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 704/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk704<F: Float>(t36700: F, t2184: F, t465: F, t7472: F, t7335: F, t7341: F, t20: F, t2018: F, t2021: F, t4720: F, t7338: F, t7491: F, t1969: F, t34846: F, t7345: F, t7927: F) -> (F, F, F, F, F, F, F, F, F) {
    let t36701 = 0.91462949374725084942e-3 * t36700;
    let t36733 = t465 * t2184;
    let t36734 = t7472 * t36733;
    let t36748 = t7335 * t7341;
    let t36752 = t4720 * t20 * t2018 * t2021;
    let t36753 = 0.15243824895787514157e-3 * t36752;
    let t36754 = t7335 * t7338;
    let t36756 = t7491 * t7341;
    let t36772 = t34846 * t1969;
    let t36796 = t7345 * t7927;
    (t36701, t36733, t36734, t36748, t36753, t36754, t36756, t36772, t36796)
}
