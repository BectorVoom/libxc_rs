//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 921/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk921(t1743: f64, t3351: f64, t498: f64, t511: f64, t7231: f64, t39199: f64, t8571: f64, t39183: f64, t40705: f64, t1981: f64, t3142: f64, t626: f64, t8512: f64) -> (f64, f64, f64, f64, f64) {
    let t45361 = t3351 * t7231 * t511 * t1743 * t498;
    let t45363 = t8571 * t39199;
    let t45365 = t8571 * t39183;
    let t45367 = t8571 * t40705;
    let t45371 = t8512 * t1981 * t3142 * t626;
    (t45361, t45363, t45365, t45367, t45371)
}
