//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 513/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk513(t1228: f64, t1518: f64, t1190: f64, t1497: f64, t205: f64, t5474: f64, t23: f64, t470: f64, t4388: f64, t589: f64, t1144: f64, t1156: f64, t1392: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5633 = t1228 * t1518;
    let t5636 = 0.12805126321218922714e0_f64 * t1190 * t1497;
    let t5637 = t5474 * t205;
    let t5647 = t470 * t23;
    let t5652 = t4388 * t589;
    let t5653 = t5652 * t1144;
    let t5656 = t1156 * t1392;
    (t5633, t5636, t5637, t5647, t5653, t5656)
}
