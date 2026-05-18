//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1452/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1452<F: Float>(t12161: F, t795: F, t12281: F, t12284: F, t12306: F, t1841: F, t1843: F, t1901: F, t1908: F, t1939: F, t2508: F, t270: F, t29324: F, t29349: F, t32340: F, t32343: F, t32351: F, t32353: F, t3723: F, t38970: F, t39188: F, t39272: F, t5269: F, t650: F, t681: F, t7129: F, t738: F, t740: F) -> F {
    let t39403 = t795 * t12161;
    let t39407 = t32340 + F::new(0.20508069947045931424e-1) * t650 * t12306 + F::new(0.34180116578409885707e-2) * t1908 * t3723 - t29324 + F::new(0.17090058289204942853e-2) * t1841 * t1843 * t39272 + F::new(0.20508069947045931424e-1) * t1939 * t3723 - F::new(0.76905262301422242837e-2) * t270 * t738 * t39188 - F::new(0.15381052460284448567e-1) * t681 * t12281 + t32343 + t29349 + t32351 - t32353 - F::new(0.15381052460284448567e-1) * t5269 * t1901 * t38970 - F::new(0.46143157380853345702e-1) * t7129 * t12284 - F::new(0.46143157380853345702e-1) * t2508 * t39403 * t740;
    t39407
}
