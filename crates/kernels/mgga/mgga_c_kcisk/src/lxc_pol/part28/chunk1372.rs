//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1372/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1372<F: Float>(t17078: F, t1772: F, t2447: F, t112192: F, t2364: F, t34146: F, t116186: F, t1849: F, t2469: F, t6667: F, t116145: F, t1799: F, t6680: F, t112184: F, t112406: F, t116836: F, t116856: F, t116882: F, t117130: F, t121140: F, t121144: F, t33031: F, t33056: F, t34192: F, t34218: F, t69891: F, t7261: F, t9652: F, t9664: F, t9922: F) -> (F, F) {
    let t121621 = t17078 * t2447 * t1772;
    let t121626 = t112192 * t2364 * t34146;
    let t121633 = t116186 * t2469 * t1849 * t6667;
    let t121645 = t1799 * t116145 * t6680;
    let t121657 = 0.80416666666666666669e-2 * t121621 * t9652 + 0.29479012345679012345e-2 * t116836 - 0.69444444444444444446e-2 * t33031 * t121626 - 0.26805555555555555556e-2 * t33056 * t121626 - 0.53611111111111111112e-2 * t33056 * t121633 - 0.15520416666666666667e-2 * t112406 * t121144 + 0.92592592592592592594e-2 * t33031 * t121140 - 0.13888888888888888889e-1 * t33031 * t121144 - 0.13888888888888888889e-1 * t33031 * t121633 + 0.88437037037037037033e-2 * t121645 - t116856 + 0.62500000000000000002e-1 * t9664 * t7261 * t112184 * t69891 + 0.8041666666666666667e-2 * t116882 * t9922 + 0.8041666666666666667e-2 * t117130 * t9922 + 0.8041666666666666667e-2 * t34192 * t34218;
    (t121645, t121657)
}
