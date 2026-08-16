//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 849/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk849(t38843: f64, t640: f64, t7553: f64, t7555: f64, t34795: f64, t529: f64, t2010: f64, t34797: f64, t2415: f64, t35220: f64, t7349: f64, t1411: f64, t7754: f64) -> (f64, f64, f64, f64) {
    let t38844 = t640 * t38843;
    let t38846 = t7553 * t7555 * t38844;
    let t38848 = t34795 * t529;
    let t38850 = t2010 * t38848 * t34797;
    let t38853 = t7349 * t2415 * t35220;
    let t38854 = 0.10248087766267884742e-3_f64 * t38853;
    let t38855 = t7754 * t1411;
    (t38846, t38850, t38854, t38855)
}
