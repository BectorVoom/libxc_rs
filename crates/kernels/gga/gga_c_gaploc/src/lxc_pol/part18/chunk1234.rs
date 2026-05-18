//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1234/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1234<F: Float>(t32482: F, t10749: F, t731: F, t23362: F, t2936: F, t5269: F, t10755: F, t5293: F, t27403: F, t954: F, t32179: F, t550: F) -> (F, F, F, F, F, F) {
    let t32483 = F::new(0.22430701504581487494e-2) * t32482;
    let t32484 = t731 * t10749;
    let t32485 = F::new(0.85450291446024714264e-3) * t32484;
    let t32488 = F::new(0.46143157380853345702e-1) * t5269 * t2936 * t23362;
    let t32490 = F::new(0.20508069947045931424e-1) * t5293 * t10755;
    let t32493 = F::new(0.15381052460284448567e-1) * t5269 * t954 * t27403;
    let t32504 = t550 * t32179;
    (t32483, t32485, t32488, t32490, t32493, t32504)
}
