//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 796/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk796<F: Float>(t11287: F, t11294: F, t11296: F, t11299: F, t11301: F, t11304: F, t11307: F, t11310: F, t8099: F, t8110: F, t8113: F, t8116: F, t8133: F) -> F {
    let t12549 = -F::cast_from(0.11853866860905349795e0_f64) * t11287 - F::cast_from(0.11113000182098765433e-1_f64) * t8099 - F::cast_from(0.74086667880658436219e-2_f64) * t8110 + F::cast_from(0.55565000910493827163e-2_f64) * t8113 + F::cast_from(0.74086667880658436217e-2_f64) * t8116 - F::cast_from(0.29634667152263374487e-1_f64) * t8133 + F::cast_from(0.16299066933744855968e0_f64) * t11294 - F::cast_from(0.29634667152263374487e-1_f64) * t11296 - F::cast_from(0.37043333940329218109e-2_f64) * t11299 - F::cast_from(0.17780800291358024692e0_f64) * t11301 - F::cast_from(0.77791001274691358028e-1_f64) * t11304 - F::cast_from(0.13335600218518518519e0_f64) * t11307 + F::cast_from(0.10001700163888888889e0_f64) * t11310;
    t12549
}
