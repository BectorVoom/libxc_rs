//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1378/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1378(t103626: f64, t7898: f64, t18210: f64, t2237: f64, t29343: f64, t102640: f64, t102642: f64, t27410: f64, t28480: f64, t29300: f64, t29393: f64, t7916: f64, t8148: f64, t98777: f64, t98795: f64, t98804: f64, t98806: f64, t98813: f64) -> f64 {
    let t103670 = t7898 * t103626;
    let t103674 = t2237 * t18210 * t29343;
    let t103686 = 0.30918233506944444444e-4_f64 * t103670 - 0.49745833333333333332e-2_f64 * t102640 + 0.23168402777777777778e-3_f64 * t103674 + 0.69505208333333333333e-3_f64 * t29393 * t7916 - 0.30891203703703703704e-3_f64 * t98777 - 0.58958024691358024689e-2_f64 * t102642 + 0.92754700520833333333e-4_f64 * t27410 * t29300 + t98795 + 0.11054629629629629629e-2_f64 * t98804 - 0.7369753086419753086e-3_f64 * t98806 - 0.37069444444444444444e-2_f64 * t28480 * t8148 + t98813;
    t103686
}
