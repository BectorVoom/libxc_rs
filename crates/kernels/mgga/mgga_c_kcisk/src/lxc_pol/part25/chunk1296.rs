//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1296/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1296<F: Float>(t4644: F, t7242: F, t7283: F, t10802: F, t112406: F, t116251: F, t116306: F, t116482: F, t116489: F, t116495: F, t116498: F, t116507: F, t116511: F, t116513: F, t116516: F, t33031: F, t33035: F, t34021: F, t4640: F, t5015: F, t9664: F) -> (F, F) {
    let t116520 = t7242 * t7283 * t4644;
    let t116527 = -0.18518518518518518519e-1 * t116482 * t33035 - 0.71481481481481481485e-2 * t116489 * t33035 - 0.20833333333333333334e-1 * t9664 * t116251 - 0.66327777777777777776e-2 * t116495 + 0.55273148148148148146e-2 * t116498 + 0.46561250000000000002e-2 * t112406 * t116306 - 0.69444444444444444446e-2 * t33031 * t5015 * t34021 * t4644 - 0.22109259259259259258e-2 * t116507 - 0.73697530864197530861e-3 * t116511 + 0.69444444444444444446e-2 * t116513 * t33035 + 0.26805555555555555556e-2 * t116516 * t33035 - 0.69444444444444444446e-2 * t33031 * t116520 + 0.46296296296296296297e-2 * t33031 * t10802 * t34021 * t4640;
    (t116520, t116527)
}
