//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1368/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1368<F: Float>(t112692: F, t415: F, t8673: F, t34073: F, t34118: F, t2063: F, t33032: F, t7242: F, t7274: F, t1849: F, t4826: F, t121477: F, t112289: F, t116513: F, t116516: F, t116703: F, t116705: F, t116723: F, t116731: F, t121246: F, t121389: F, t121471: F, t22289: F, t33031: F, t34016: F, t34027: F, t34037: F, t35123: F, t5015: F, t9649: F) -> (F, F, F, F) {
    let t121520 = t415 * t112692 * t8673;
    let t121522 = t34073 * t34118;
    let t121531 = t7242 * t33032 * t2063 * t7274;
    let t121544 = t4826 * t1849;
    let t121546 = t7242 * t121544 * t121477;
    let t121553 = t116703 + t116705 - 0.7369753086419753086e-3 * t116723 + 0.49745833333333333332e-2 * t121520 - 0.23148148148148148149e-2 * t121522 + 0.44229166666666666667e-1 * t9649 * t121246 - 0.11054629629629629629e-2 * t116731 + 0.69444444444444444446e-2 * t112289 * t35123 + 0.69444444444444444446e-2 * t33031 * t121531 - 0.13888888888888888889e-1 * t33031 * t121389 + 0.69444444444444444447e-2 * t116513 * t34027 - 0.69444444444444444446e-2 * t33031 * t5015 * t34037 * t121471 + 0.26805555555555555557e-2 * t116516 * t34027 - 0.69444444444444444446e-2 * t33031 * t121546 - 0.20833333333333333334e-1 * t33031 * t7242 * t34016 * t22289;
    (t121520, t121531, t121546, t121553)
}
