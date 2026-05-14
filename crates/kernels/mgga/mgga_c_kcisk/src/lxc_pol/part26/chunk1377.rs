//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1377/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1377<F: Float>(t33767: F, t33873: F, t2737: F, t34935: F, t4419: F, t115085: F, t115312: F, t115471: F, t115535: F, t120101: F, t32354: F, t32436: F, t33832: F, t33837: F, t33941: F, t34931: F, t34936: F, t34990: F, t9512: F, t9529: F, t9536: F, t9855: F) -> (F,) {
    let t120243 = t33767 * t33873;
    let t120248 = t2737 * t4419 * t34935;
    let t120252 = -0.20833333333333333334e-1 * t33941 * t33832 - 0.10416666666666666667e-1 * t33941 * t33837 - 0.40208333333333333335e-2 * t115085 * t33837 - 0.10416666666666666667e-1 * t32436 * t34931 - 0.10416666666666666667e-1 * t32354 * t34931 - 0.10416666666666666667e-1 * t9536 * t120101 + 0.40208333333333333334e-2 * t115471 * t9855 + 0.40208333333333333334e-2 * t115535 * t9855 + t115312 + 0.13402777777777777778e-2 * t120243 + 0.27777777777777777779e-1 * t9529 * t34936 - 0.34722222222222222223e-2 * t120248 + 0.52083333333333333333e-2 * t9512 * t34990;
    (t120252,)
}
