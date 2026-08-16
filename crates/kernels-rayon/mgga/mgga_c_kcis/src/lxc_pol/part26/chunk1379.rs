//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1379/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1379(t28423: f64, t8151: f64, t103662: f64, t2237: f64, t102649: f64, t102653: f64, t28403: f64, t29324: f64, t29332: f64, t7895: f64, t8144: f64, t94390: f64, t98815: f64, t98818: f64, t98820: f64, t98823: f64, t98825: f64) -> f64 {
    let t103693 = t8151 * t28423;
    let t103698 = t2237 * t103662;
    let t103700 = t98815 + t98818 + t98820 - 0.16581944444444444444e-2_f64 * t102649 - 0.185671721767578125e-4_f64 * t94390 * t29324 - 0.13901041666666666667e-2_f64 * t7895 * t29332 - t98823 - 0.12356481481481481481e-2_f64 * t103693 - 0.49745833333333333332e-2_f64 * t102653 + 0.13901041666666666667e-2_f64 * t8144 * t28403 + t98825 - 0.46336805555555555557e-3_f64 * t103698;
    t103700
}
