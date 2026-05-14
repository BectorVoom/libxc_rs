//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1363/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1363<F: Float>(t112266: F, t112406: F, t112709: F, t116516: F, t116960: F, t121010: F, t121052: F, t121116: F, t121374: F, t121381: F, t121385: F, t121389: F, t121399: F, t121405: F, t32942: F, t32990: F, t33056: F, t34013: F, t34023: F, t35097: F, t35123: F, t9664: F, t9672: F) -> (F,) {
    let t121411 = 0.13265555555555555555e-1 * t121374 + 0.26805555555555555556e-2 * t116516 * t34013 + 0.26805555555555555556e-2 * t112709 * t35123 + 0.46561250000000000002e-2 * t112406 * t121381 - 0.40208333333333333333e-2 * t33056 * t121385 - 0.53611111111111111112e-2 * t33056 * t121389 + 0.69444444444444444446e-2 * t112266 * t35123 + 0.10416666666666666667e-1 * t121116 * t9672 + 0.10416666666666666667e-1 * t9664 * t121010 + 0.27636574074074074073e-2 * t121399 + 0.13402777777777777778e-2 * t33056 * t121052 + 0.69444444444444444447e-2 * t116960 * t34023 - 0.22109259259259259259e-2 * t121405 - 0.20833333333333333334e-1 * t32942 * t35097 - 0.20833333333333333334e-1 * t32990 * t35097;
    (t121411,)
}
