//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1274/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1274<F: Float>(t12147: F, t28550: F, t7908: F, t98487: F, t16937: F, t28488: F, t2237: F, t98537: F, t28461: F, t7901: F, t98795: F, t98797: F, t98800: F, t98804: F, t98806: F, t98809: F) -> F {
    let t98813 = F::cast_from(0.15445601851851851852e-3_f64) * t7908 * t12147 * t28550;
    let t98815 = F::cast_from(0.15445601851851851852e-3_f64) * t7908 * t98487;
    let t98818 = F::cast_from(0.30891203703703703704e-3_f64) * t7908 * t16937 * t28488;
    let t98820 = F::cast_from(0.46336805555555555556e-3_f64) * t2237 * t98537;
    let t98821 = t98795 - F::cast_from(0.22109259259259259258e-2_f64) * t98797 + F::cast_from(0.11054629629629629629e-2_f64) * t98800 + F::cast_from(0.13901041666666666667e-2_f64) * t28461 * t7901 + F::cast_from(0.55273148148148148147e-3_f64) * t98804 - F::cast_from(0.3684876543209876543e-3_f64) * t98806 - F::cast_from(0.13265555555555555555e-1_f64) * t98809 + t98813 + t98815 + t98818 + t98820;
    t98821
}
