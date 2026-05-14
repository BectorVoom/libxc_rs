//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1283/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1283<F: Float>(t116223: F, t1799: F, t5187: F, t34107: F, t5195: F, t17742: F, t9679: F, t17753: F, t34159: F, t116201: F, t116206: F, t116211: F, t116212: F, t116220: F, t17301: F, t32938: F, t32959: F, t32990: F, t33031: F, t34073: F, t34125: F, t34133: F, t34225: F, t9652: F, t9664: F, t9672: F) -> (F, F, F, F, F) {
    let t116225 = t1799 * t116223 * t5187;
    let t116228 = t1799 * t34107 * t5195;
    let t116231 = t1799 * t9679 * t17742;
    let t116236 = t1799 * t34159 * t17753;
    let t116242 = 0.20833333333333333334e-1 * t116201 * t9672 - 0.33163888888888888888e-2 * t116206 + 0.20833333333333333334e-1 * t116201 * t9652 + t116211 - 0.13888888888888888889e-1 * t33031 * t116212 * t17301 + 0.13888888888888888889e-1 * t32990 * t34133 + 0.20833333333333333334e-1 * t9664 * t116220 - 0.33163888888888888888e-2 * t116225 + 0.22109259259259259258e-2 * t116228 - 0.3684876543209876543e-3 * t116231 - 0.34722222222222222223e-2 * t34073 * t32959 + 0.33163888888888888888e-2 * t116236 + 0.55555555555555555558e-1 * t34125 * t32938 + 0.21444444444444444446e-1 * t34225 * t32938;
    (t116225, t116228, t116231, t116236, t116242)
}
