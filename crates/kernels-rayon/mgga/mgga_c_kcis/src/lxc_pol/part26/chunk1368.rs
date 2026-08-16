//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1368/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1368(t28426: f64, t28544: f64, t102381: f64, t102384: f64, t27410: f64, t28397: f64, t28403: f64, t29324: f64, t94524: f64, t94537: f64, t94539: f64, t98522: f64, t98525: f64, t98538: f64) -> f64 {
    let t103483 = t28544 * t28426;
    let t103494 = 0.16581944444444444444e-2_f64 * t102381 - 0.2782641015625e-3_f64 * t27410 * t29324 - 0.16489724537037037037e-3_f64 * t103483 - 0.24872916666666666666e-2_f64 * t102384 - 0.20612155671296296296e-4_f64 * t94524 + 0.22109259259259259259e-2_f64 * t98522 + 0.18550940104166666667e-3_f64 * t28397 * t28403 - 0.30891203703703703704e-3_f64 * t98525 + 0.55273148148148148147e-3_f64 * t94537 - 0.36848765432098765431e-3_f64 * t94539 + 0.61836467013888888889e-4_f64 * t98538;
    t103494
}
