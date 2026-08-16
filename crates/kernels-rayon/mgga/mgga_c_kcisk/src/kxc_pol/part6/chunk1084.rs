//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1084/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1084(t294: f64, t31804: f64, t15452: f64, t15463: f64, t15473: f64, t15763: f64, t28152: f64, t30148: f64, t30149: f64, t30151: f64, t30173: f64, t30176: f64, t30181: f64, t31803: f64) -> f64 {
    let t31805 = t294 * t31804;
    let t31806 = 3.0_f64 / 16.0_f64 * t31805;
    let t31807 = -t30148 + t15452 + t30149 + t30151 + 3.0_f64 * t28152 - t15463 + t30173 - t30176 - t30181 - t31803 - t15473 + t15763 - t31806;
    t31807
}
