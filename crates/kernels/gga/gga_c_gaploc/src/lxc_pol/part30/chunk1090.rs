//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1090/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1090<F: Float>(t32214: F, t7290: F, t2530: F, t8469: F, t2508: F, t2580: F, t24339: F, t935: F, t10782: F, t1865: F, t11004: F, t21556: F, t3420: F, t10773: F, t7137: F, t3448: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t32215 = t7290 * t32214;
    let t32219 = t8469 * t2530;
    let t32222 = 0.30762104920568897134e-1 * t2508 * t2580 * t32219;
    let t32223 = t24339 * t935;
    let t32226 = 0.15381052460284448567e-1 * t2508 * t2580 * t32223;
    let t32230 = t10782 * t1865;
    let t32234 = t11004 * t1865;
    let t32241 = 0.20508069947045931424e-1 * t21556 * t3420;
    let t32243 = 0.20508069947045931424e-1 * t7137 * t10773;
    let t32245 = 0.41016139894091862846e-1 * t21556 * t3448;
    (t32215, t32219, t32222, t32223, t32226, t32230, t32234, t32241, t32243, t32245)
}
