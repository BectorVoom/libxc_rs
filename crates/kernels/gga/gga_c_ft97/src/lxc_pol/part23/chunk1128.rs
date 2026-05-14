//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1128/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1128<F: Float>(t1113: F, t226: F, t24324: F, t24330: F, t27515: F, t10915: F, t229: F, t6808: F, t6809: F, t96535: F, t37481: F, t6789: F, t6793: F, t108572: F, t70: F, t27661: F) -> (F, F, F, F, F, F, F) {
    let t108880 = t1113 * t226;
    let t108940 = t24324 * t24330 * t27515;
    let t108972 = t229 * t10915;
    let t109002 = t6808 * t96535 * t6809;
    let t109008 = t37481 * t6789 * t6793;
    let t109014 = t108572 * t70;
    let t109015 = t109014 * t27661;
    (t108880, t108940, t108972, t109002, t109008, t109014, t109015)
}
