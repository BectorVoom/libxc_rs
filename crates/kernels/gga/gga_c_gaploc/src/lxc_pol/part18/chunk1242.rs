//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1242/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1242<F: Float>(t10669: F, t10674: F, t169: F, t1908: F, t1935: F, t1939: F, t270: F, t29471: F, t29473: F, t29476: F, t299: F, t32313: F, t32585: F, t32588: F, t32591: F, t32594: F, t32597: F, t3434: F, t3452: F, t650: F, t706: F) -> F {
    let t32598 = -F::cast_from(0.20508069947045931424e-1_f64) * t650 * t10669 - F::cast_from(0.76905262301422242837e-2_f64) * t1935 * t3452 + F::cast_from(0.76905262301422242837e-2_f64) * t270 * t706 * t32313 * t169 * t299 - F::cast_from(0.34180116578409885707e-2_f64) * t1908 * t3452 + F::cast_from(0.20508069947045931424e-1_f64) * t650 * t10674 + F::cast_from(0.20508069947045931424e-1_f64) * t1939 * t3434 - t29471 + t29473 - t32585 + t32588 + t32591 - t32594 - t29476 + t32597;
    t32598
}
