//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1274/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1274(t12147: f64, t28550: f64, t7908: f64, t98487: f64, t16937: f64, t28488: f64, t2237: f64, t98537: f64, t28461: f64, t7901: f64, t98795: f64, t98797: f64, t98800: f64, t98804: f64, t98806: f64, t98809: f64) -> f64 {
    let t98813 = 0.15445601851851851852e-3_f64 * t7908 * t12147 * t28550;
    let t98815 = 0.15445601851851851852e-3_f64 * t7908 * t98487;
    let t98818 = 0.30891203703703703704e-3_f64 * t7908 * t16937 * t28488;
    let t98820 = 0.46336805555555555556e-3_f64 * t2237 * t98537;
    let t98821 = t98795 - 0.22109259259259259258e-2_f64 * t98797 + 0.11054629629629629629e-2_f64 * t98800 + 0.13901041666666666667e-2_f64 * t28461 * t7901 + 0.55273148148148148147e-3_f64 * t98804 - 0.3684876543209876543e-3_f64 * t98806 - 0.13265555555555555555e-1_f64 * t98809 + t98813 + t98815 + t98818 + t98820;
    t98821
}
