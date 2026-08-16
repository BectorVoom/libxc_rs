//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 659/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk659(t31: f64, t574: f64, t640: f64, t34795: f64, t529: f64, t1411: f64, t7754: f64, t1540: f64, t880: f64, t49: f64, t2410: f64, t7228: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38843 = t574 * t31;
    let t38844 = t640 * t38843;
    let t38848 = t34795 * t529;
    let t38855 = t7754 * t1411;
    let t38973 = t1540 * t880;
    let t39116 = t49 * t529;
    let t39207 = t2410 * t7228;
    (t38844, t38848, t38855, t38973, t39116, t39207)
}
