//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 796/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk796(t11287: f64, t11294: f64, t11296: f64, t11299: f64, t11301: f64, t11304: f64, t11307: f64, t11310: f64, t8099: f64, t8110: f64, t8113: f64, t8116: f64, t8133: f64) -> f64 {
    let t12549 = -0.11853866860905349795e0_f64 * t11287 - 0.11113000182098765433e-1_f64 * t8099 - 0.74086667880658436219e-2_f64 * t8110 + 0.55565000910493827163e-2_f64 * t8113 + 0.74086667880658436217e-2_f64 * t8116 - 0.29634667152263374487e-1_f64 * t8133 + 0.16299066933744855968e0_f64 * t11294 - 0.29634667152263374487e-1_f64 * t11296 - 0.37043333940329218109e-2_f64 * t11299 - 0.17780800291358024692e0_f64 * t11301 - 0.77791001274691358028e-1_f64 * t11304 - 0.13335600218518518519e0_f64 * t11307 + 0.10001700163888888889e0_f64 * t11310;
    t12549
}
