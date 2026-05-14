//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1101/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1101<F: Float>(t108572: F, t70: F, t27661: F, t27658: F, t52595: F, t6757: F, t24265: F, t27552: F, t697: F, t2446: F, t420: F, t24276: F, t27533: F, t96419: F, t24323: F, t3766: F) -> (F, F, F, F, F, F, F) {
    let t109014 = t108572 * t70;
    let t109015 = t109014 * t27661;
    let t109017 = 0.10091343167942740398e-3 * t27658 * t109015;
    let t109024 = t6757 * t52595;
    let t109030 = 0.29693535778629056444e-3 * t24265 * t697 * t27552;
    let t109033 = t420 * t2446;
    let t109038 = t24276 * t96419 * t27533;
    let t109047 = t3766 * t24323;
    (t109015, t109017, t109024, t109030, t109033, t109038, t109047)
}
