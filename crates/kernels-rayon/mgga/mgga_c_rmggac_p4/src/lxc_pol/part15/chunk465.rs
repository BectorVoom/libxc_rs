//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 465/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk465(t1228: f64, t1518: f64, t1190: f64, t1497: f64, t219: f64, t4467: f64, t4462: f64, t612: f64, t1522: f64, t4555: f64, t608: f64, t1477: f64, t4559: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5633 = t1228 * t1518;
    let t5636 = 0.12805126321218922714e0_f64 * t1190 * t1497;
    let t5672 = t4467 * t219;
    let t5677 = t4462 * t612;
    let t5681 = 0.25610252642437845428e0_f64 * t1228 * t1522;
    let t5685 = t4555 * t608;
    let t5693 = 0.25610252642437845428e0_f64 * t4559 * t1477;
    (t5633, t5636, t5672, t5677, t5681, t5685, t5693)
}
