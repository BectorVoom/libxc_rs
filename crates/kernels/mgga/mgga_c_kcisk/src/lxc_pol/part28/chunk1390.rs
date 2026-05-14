//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1390/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1390<F: Float>(t35143: F, t9660: F, t34107: F, t5054: F, t6676: F, t23953: F, t33017: F, t117192: F, t117194: F, t117195: F, t121253: F, t121258: F, t121368: F, t121479: F, t1693: F, t17010: F, t20: F, t23220: F, t2454: F, t2785: F, t33031: F, t33056: F, t34016: F, t34037: F, t7201: F, t7234: F, t7242: F, t9664: F, t9931: F) -> (F, F, F) {
    let t121999 = t35143 * t9660;
    let t122008 = t5054 * t34107 * t6676;
    let t122015 = t5054 * t33017 * t23953;
    let t122020 = 0.69444444444444444446e-2 * t33031 * t7242 * t34037 * t23220 + 0.13402777777777777778e-2 * t33056 * t121368 - 0.46296296296296296297e-2 * t33031 * t7234 * t34016 * t23220 + 0.17870370370370370371e-2 * t33056 * t121479 + 0.18518518518518518519e-1 * t121999 + t117192 + t117194 + 0.22109259259259259259e-2 * t117195 + 0.55555555555555555558e-1 * t1693 * t7201 * t2454 * t20 * t2785 + 0.55273148148148148147e-2 * t122008 - 0.10416666666666666667e-1 * t9664 * t121253 - 0.20833333333333333334e-1 * t9664 * t121258 - 0.27636574074074074073e-2 * t122015 + 0.55555555555555555558e-1 * t17010 * t9931 * t2785;
    (t122008, t122015, t122020)
}
