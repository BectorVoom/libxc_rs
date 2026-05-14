//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1178/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1178<F: Float>(t9512: F, t9532: F, t32139: F, t32151: F, t32153: F, t32433: F, t32436: F, t32439: F, t32443: F, t32447: F, t32461: F, t32468: F, t32474: F, t9519: F, t9529: F, t9536: F, t9539: F, t9544: F) -> (F, F) {
    let t32477 = t9512 * t9532;
    let t32479 = -0.27777777777777777778e-1 * t9529 * t9544 - 0.27777777777777777778e-1 * t9529 * t9519 - 0.10722222222222222222e-1 * t32433 * t9519 - 0.34722222222222222222e-2 * t32436 * t9539 - 0.40208333333333333334e-2 * t32439 * t32443 + 0.34722222222222222222e-2 * t9536 * t32447 - 0.10416666666666666667e-1 * t9536 * t32443 + 0.10416666666666666667e-1 * t9512 * t9519 - 0.17411041666666666666e-2 * t32139 + 0.17411041666666666666e-2 * t32151 + 0.23214722222222222222e-2 * t32153 + 0.34722222222222222222e-2 * t9536 * t32461 + 0.34722222222222222222e-2 * t9536 * t32468 + 0.13402777777777777778e-2 * t32439 * t32468 + 0.40208333333333333334e-2 * t32474 * t9519 - 0.34722222222222222222e-2 * t32477;
    (t32477, t32479)
}
