//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 644/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk644(t1615: f64, t6207: f64, t1592: f64, t1617: f64, t2110: f64, t4117: f64, t4399: f64, t4409: f64, t4414: f64, t5750: f64, t5754: f64, t5759: f64, t5762: f64, t5764: f64, t5766: f64, t5771: f64, t5774: f64, t5777: f64, t5783: f64, t5873: f64, t5878: f64, t5883: f64, t5892: f64, t6193: f64) -> (f64, f64) {
    let t6208 = t6207 * t1615;
    let t6219 = -0.66725e-1_f64 * t6193 * t1617 - 0.66725e-1_f64 * t4409 * t2110 - 0.17411041666666666666e-2_f64 * t5750 + 0.11607361111111111111e-2_f64 * t5754 - 0.30952962962962962962e-2_f64 * t5759 + 0.11607361111111111111e-2_f64 * t5762 + 0.77382407407407407407e-3_f64 * t5764 - 0.11607361111111111111e-2_f64 * t5766 - t4399 + 0.11607361111111111111e-2_f64 * t4117 - 0.30952962962962962962e-2_f64 * t5771 + 0.11607361111111111111e-2_f64 * t5774 + 0.890445125e-2_f64 * t4414 * t6208 + 0.66725e-1_f64 * t1592 * t6208 + 0.11607361111111111111e-2_f64 * t5777 - 0.23214722222222222222e-2_f64 * t5783 + 0.17411041666666666666e-2_f64 * t5873 - 0.38691203703703703703e-3_f64 * t5878 - 0.11607361111111111111e-2_f64 * t5883 - 0.46429444444444444443e-2_f64 * t5892;
    (t6208, t6219)
}
