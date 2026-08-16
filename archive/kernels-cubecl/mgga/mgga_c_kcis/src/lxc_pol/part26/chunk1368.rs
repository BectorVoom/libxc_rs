//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1368/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1368<F: Float>(t28426: F, t28544: F, t102381: F, t102384: F, t27410: F, t28397: F, t28403: F, t29324: F, t94524: F, t94537: F, t94539: F, t98522: F, t98525: F, t98538: F) -> F {
    let t103483 = t28544 * t28426;
    let t103494 = F::cast_from(0.16581944444444444444e-2_f64) * t102381 - F::cast_from(0.2782641015625e-3_f64) * t27410 * t29324 - F::cast_from(0.16489724537037037037e-3_f64) * t103483 - F::cast_from(0.24872916666666666666e-2_f64) * t102384 - F::cast_from(0.20612155671296296296e-4_f64) * t94524 + F::cast_from(0.22109259259259259259e-2_f64) * t98522 + F::cast_from(0.18550940104166666667e-3_f64) * t28397 * t28403 - F::cast_from(0.30891203703703703704e-3_f64) * t98525 + F::cast_from(0.55273148148148148147e-3_f64) * t94537 - F::cast_from(0.36848765432098765431e-3_f64) * t94539 + F::cast_from(0.61836467013888888889e-4_f64) * t98538;
    t103494
}
