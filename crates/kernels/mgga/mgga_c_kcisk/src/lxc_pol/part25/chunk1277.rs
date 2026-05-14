//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1277/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1277<F: Float>(t116126: F, t34203: F, t5074: F, t112180: F, t112216: F, t112244: F, t112387: F, t116118: F, t116120: F, t116123: F, t32893: F, t32921: F, t32990: F, t33005: F, t34122: F, t34192: F, t34218: F, t34261: F, t9672: F, t9922: F, t9940: F) -> (F, F) {
    let t116127 = 0.22109259259259259258e-2 * t116126;
    let t116129 = t5074 * t34203;
    let t116130 = 0.14739506172839506172e-2 * t116129;
    let t116131 = 0.10416666666666666667e-1 * t34122 * t32893 - 0.120625e-1 * t34192 * t33005 + 0.10416666666666666667e-1 * t112216 * t9922 + 0.20833333333333333334e-1 * t112244 * t9922 + 0.8041666666666666667e-2 * t32921 * t34218 + 0.10416666666666666667e-1 * t112387 * t9940 + 0.20833333333333333334e-1 * t32990 * t34261 + t116118 + 0.20833333333333333334e-1 * t116120 * t9672 - 0.55555555555555555558e-1 * t116123 * t9672 - t116127 - 0.22109259259259259258e-2 * t112180 + t116130;
    (t116129, t116131)
}
