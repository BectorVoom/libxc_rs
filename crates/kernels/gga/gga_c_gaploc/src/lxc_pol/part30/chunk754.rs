//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 754/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk754<F: Float>(t1850: F, t1908: F, t1935: F, t1939: F, t2533: F, t2583: F, t2587: F, t270: F, t5269: F, t650: F, t681: F, t7066: F, t7070: F, t7115: F, t7125: F, t7129: F, t7133: F, t7137: F, t938: F, t949: F) -> F {
    let t7140 = -F::new(0.64087718584518535698e-3) * t7066 - F::new(0.17090058289204942853e-2) * t1850 * t7070 + F::new(0.34180116578409885707e-2) * t1908 * t938 - F::new(0.20508069947045931424e-1) * t1939 * t949 - F::new(0.20508069947045931424e-1) * t650 * t2587 - F::new(0.76905262301422242837e-2) * t1935 * t949 - F::new(0.15381052460284448567e-1) * t681 * t2587 - F::new(0.34180116578409885707e-2) * t1908 * t949 + F::new(0.15381052460284448567e-1) * t681 * t2533 + F::new(0.76905262301422242837e-2) * t270 * t7115 + F::new(0.20508069947045931424e-1) * t1939 * t938 + F::new(0.20508069947045931424e-1) * t650 * t2533 + F::new(0.76905262301422242837e-2) * t1935 * t938 - F::new(0.76905262301422242837e-2) * t270 * t7125 + F::new(0.30762104920568897134e-1) * t7129 * t2583 - F::new(0.15381052460284448567e-1) * t5269 * t7133 + F::new(0.41016139894091862847e-1) * t7137 * t2583;
    t7140
}
