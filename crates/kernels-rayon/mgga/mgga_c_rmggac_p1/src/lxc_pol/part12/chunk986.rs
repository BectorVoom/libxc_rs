//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 986/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk986(t39671: f64, t7829: f64, t39685: f64, t39675: f64, t7782: f64, t39681: f64, t40735: f64, t7788: f64, t40135: f64, t40739: f64, t2392: f64, t848: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40850 = t7829 * t39671;
    let t40852 = t7829 * t39685;
    let t40854 = t7782 * t39675;
    let t40856 = t7782 * t39681;
    let t40858 = t7788 * t40735;
    let t40860 = t7788 * t40135;
    let t40862 = t7782 * t40739;
    let t40864 = t2392 * t848;
    (t40850, t40852, t40854, t40856, t40858, t40860, t40862, t40864)
}
