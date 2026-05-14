//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1129/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1129<F: Float>(t109015: F, t27658: F, t24265: F, t27552: F, t697: F, t2446: F, t420: F, t24276: F, t27533: F, t96419: F, t24330: F, t27546: F, t27548: F, t17839: F, t6032: F, t17836: F) -> (F, F, F, F, F, F, F) {
    let t109017 = 0.10091343167942740398e-3 * t27658 * t109015;
    let t109030 = 0.29693535778629056444e-3 * t24265 * t697 * t27552;
    let t109033 = t420 * t2446;
    let t109038 = t24276 * t96419 * t27533;
    let t109055 = 0.51074886703703703704e-1 * t27546 * t24330 * t27548;
    let t109063 = t6032 * t17839;
    let t109064 = t17836 * t109063;
    (t109017, t109030, t109033, t109038, t109055, t109063, t109064)
}
