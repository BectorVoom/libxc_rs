//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 904/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk904<F: Float>(t14163: F, t27753: F, t3880: F, t6135: F, t10007: F, t1882: F, t6863: F, t6854: F, t1449: F, t2360: F, t3886: F, t14182: F, t265: F, t27742: F, t729: F, t24789: F, t3898: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t28375 = t14163 * t27753;
    let t28378 = t6135 * t3880;
    let t28379 = t10007 * t28378;
    let t28382 = t1882 * t6863;
    let t28384 = t1882 * t6854;
    let t28386 = t1449 * t2360;
    let t28387 = t28386 * t3886;
    let t28388 = t14182 * t28387;
    let t28392 = t729 * t265 * t27742;
    let t28395 = t24789 * t3898;
    (t28375, t28378, t28379, t28382, t28384, t28386, t28387, t28388, t28392, t28395)
}
