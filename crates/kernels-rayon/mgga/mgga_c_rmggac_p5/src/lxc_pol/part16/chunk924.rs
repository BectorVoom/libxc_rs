//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 924/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk924(t39199: f64, t8571: f64, t39183: f64, t40705: f64, t1981: f64, t3142: f64, t626: f64, t8512: f64, t39705: f64, t8650: f64, t1502: f64, t2318: f64, t34975: f64, t34976: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45363 = t8571 * t39199;
    let t45365 = t8571 * t39183;
    let t45367 = t8571 * t40705;
    let t45371 = t8512 * t1981 * t3142 * t626;
    let t45374 = t39705 * t8650;
    let t45381 = t34975 * t34976 * t2318 * t1502;
    (t45363, t45365, t45367, t45371, t45374, t45381)
}
