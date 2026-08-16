//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 949/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk949(t39681: f64, t7782: f64, t40735: f64, t7788: f64, t40135: f64, t40739: f64, t2392: f64, t848: f64, t262: f64, t40488: f64, t7835: f64, t39373: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40856 = t7782 * t39681;
    let t40858 = t7788 * t40735;
    let t40860 = t7788 * t40135;
    let t40862 = t7782 * t40739;
    let t40864 = t2392 * t848;
    let t40865 = t262 * t40864;
    let t40866 = t7782 * t40865;
    let t40868 = t7835 * t40488;
    let t40870 = t7835 * t39373;
    (t40856, t40858, t40860, t40862, t40864, t40865, t40866, t40868, t40870)
}
