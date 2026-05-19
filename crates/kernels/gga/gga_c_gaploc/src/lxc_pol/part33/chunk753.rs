//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 753/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk753<F: Float>(t1850: F, t1908: F, t1935: F, t1939: F, t2533: F, t2583: F, t2587: F, t270: F, t5269: F, t650: F, t681: F, t7066: F, t7070: F, t7115: F, t7125: F, t7129: F, t7133: F, t7137: F, t938: F, t949: F) -> F {
    let t7140 = -F::cast_from(0.64087718584518535698e-3_f64) * t7066 - F::cast_from(0.17090058289204942853e-2_f64) * t1850 * t7070 + F::cast_from(0.34180116578409885707e-2_f64) * t1908 * t938 - F::cast_from(0.20508069947045931424e-1_f64) * t1939 * t949 - F::cast_from(0.20508069947045931424e-1_f64) * t650 * t2587 - F::cast_from(0.76905262301422242837e-2_f64) * t1935 * t949 - F::cast_from(0.15381052460284448567e-1_f64) * t681 * t2587 - F::cast_from(0.34180116578409885707e-2_f64) * t1908 * t949 + F::cast_from(0.15381052460284448567e-1_f64) * t681 * t2533 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t7115 + F::cast_from(0.20508069947045931424e-1_f64) * t1939 * t938 + F::cast_from(0.20508069947045931424e-1_f64) * t650 * t2533 + F::cast_from(0.76905262301422242837e-2_f64) * t1935 * t938 - F::cast_from(0.76905262301422242837e-2_f64) * t270 * t7125 + F::cast_from(0.30762104920568897134e-1_f64) * t7129 * t2583 - F::cast_from(0.15381052460284448567e-1_f64) * t5269 * t7133 + F::cast_from(0.41016139894091862847e-1_f64) * t7137 * t2583;
    t7140
}
