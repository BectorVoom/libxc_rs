//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1449/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1449(t12161: f64, t795: f64, t12281: f64, t12284: f64, t12306: f64, t1841: f64, t1843: f64, t1901: f64, t1908: f64, t1939: f64, t2508: f64, t270: f64, t29324: f64, t29349: f64, t32340: f64, t32343: f64, t32351: f64, t32353: f64, t3723: f64, t38970: f64, t39188: f64, t39272: f64, t5269: f64, t650: f64, t681: f64, t7129: f64, t738: f64, t740: f64) -> f64 {
    let t39403 = t795 * t12161;
    let t39407 = t32340 + 0.20508069947045931424e-1_f64 * t650 * t12306 + 0.34180116578409885707e-2_f64 * t1908 * t3723 - t29324 + 0.17090058289204942853e-2_f64 * t1841 * t1843 * t39272 + 0.20508069947045931424e-1_f64 * t1939 * t3723 - 0.76905262301422242837e-2_f64 * t270 * t738 * t39188 - 0.15381052460284448567e-1_f64 * t681 * t12281 + t32343 + t29349 + t32351 - t32353 - 0.15381052460284448567e-1_f64 * t5269 * t1901 * t38970 - 0.46143157380853345702e-1_f64 * t7129 * t12284 - 0.46143157380853345702e-1_f64 * t2508 * t39403 * t740;
    t39407
}
