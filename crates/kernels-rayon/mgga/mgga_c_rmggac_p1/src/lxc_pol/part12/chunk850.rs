//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 850/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk850(t2010: f64, t38855: f64, t7756: f64, t34715: f64, t8465: f64, t35215: f64, t35623: f64, t7349: f64, t7760: f64, t8342: f64, t2415: f64, t35210: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38857 = t2010 * t38855 * t7756;
    let t38858 = 0.72042316457491791906e-3_f64 * t38857;
    let t38860 = t2010 * t8465 * t34715;
    let t38861 = 0.72042316457491791906e-3_f64 * t38860;
    let t38863 = t2010 * t8465 * t35215;
    let t38864 = 0.72042316457491791906e-3_f64 * t38863;
    let t38866 = t2010 * t8465 * t35623;
    let t38869 = t7349 * t8342 * t7760;
    let t38870 = 0.10248087766267884742e-3_f64 * t38869;
    let t38872 = t7349 * t2415 * t35210;
    (t38858, t38861, t38864, t38866, t38870, t38872)
}
