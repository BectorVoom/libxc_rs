//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 851/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk851(t38872: f64, t7487: f64, t8466: f64, t35207: f64, t8469: f64, t1591: f64, t2046: f64, t2050: f64, t31: f64, t34799: f64, t34803: f64, t38833: f64, t38838: f64, t38841: f64, t38846: f64, t38850: f64, t38854: f64, t38858: f64, t38861: f64, t38864: f64, t38866: f64, t38870: f64) -> f64 {
    let t38873 = 0.10248087766267884742e-3_f64 * t38872;
    let t38874 = t7487 * t8466;
    let t38876 = t35207 * t8469;
    let t38881 = t2046 * t2050 * t1591 * t31;
    let t38882 = 0.43368970657079495312e-4_f64 * t38881;
    let t38883 = -0.14408463291498358381e-2_f64 * t34799 + 0.30487649791575028314e-3_f64 * t38833 + t38838 - 0.43368970657079495312e-4_f64 * t38841 - 0.43368970657079495312e-4_f64 * t38846 - 0.72042316457491791906e-3_f64 * t38850 - t38854 + t38858 + t38861 + t38864 + 0.36021158228745895953e-3_f64 * t38866 - t38870 - t38873 - 0.19211284388664477842e-2_f64 * t38874 + 0.46116394948205481339e-3_f64 * t38876 - 0.2666855806192877858e0_f64 * t34803 + t38882;
    t38883
}
