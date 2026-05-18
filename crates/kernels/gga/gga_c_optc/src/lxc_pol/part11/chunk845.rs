//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 845/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk845<F: Float>(t9522: F, t9530: F, t16247: F, t85: F, t9701: F, t9703: F, t9705: F, t6465: F, t6477: F, t6750: F, t6753: F, t6771: F, t6811: F) -> (F, F, F, F, F, F, F) {
    let t16336 = F::new(3.0) * t9522;
    let t16337 = F::new(0.32530742648344572643e-1) * t9530;
    let t16339 = F::new(0.19751789702565206229e-1) * t16247 * t85;
    let t16340 = F::new(60.0) * t9701;
    let t16341 = F::new(36.0) * t9703;
    let t16342 = F::new(96.0) * t9705;
    let t16343 = -t6750 + t6753 + t6465 + t6771 + t16336 + t16337 + t16339 + t6811 + t6477 + t16340 + t16341 + t16342;
    (t16336, t16337, t16339, t16340, t16341, t16342, t16343)
}
