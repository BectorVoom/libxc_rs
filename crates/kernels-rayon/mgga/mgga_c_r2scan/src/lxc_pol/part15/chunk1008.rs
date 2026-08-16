//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1008/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1008(t10846: f64, t10850: f64, t10854: f64, t10857: f64, t10864: f64, t10867: f64, t11817: f64, t11819: f64, t11822: f64, t11826: f64, t11831: f64, t1584: f64, t3597: f64) -> (f64, f64) {
    let t11833 = 0.23804984598836975486e-2_f64 * t11817 + 0.54878743191129263322e-1_f64 * t11819 + 0.65495539973149862688e-2_f64 * t11822 + 0.65495539973149862688e-2_f64 * t11826 - 0.23287303101564395623e-1_f64 * t10846 - 0.69861909304693186869e-1_f64 * t10850 - t10854 - 0.48787202696913915093e-2_f64 * t10857 + 0.21831846657716620896e-2_f64 * t11831 + t10864 + t10867;
    let t11835 = t1584 * t3597;
    (t11833, t11835)
}
