//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 509/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk509(t495: f64, t5542: f64, t1173: f64, t615: f64, t1525: f64, t461: f64, t1175: f64, t1240: f64, t1510: f64, t4559: f64, t1182: f64, t589: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5543 = t5542 * t495;
    let t5554 = t1173 * t615;
    let t5555 = t5554 * t495;
    let t5558 = t461 * t1525;
    let t5561 = t615 * t1175;
    let t5564 = t1525 * t495;
    let t5567 = t615 * t1240;
    let t5571 = 0.25610252642437845428e0_f64 * t4559 * t1510;
    let t5572 = t589 * t1182;
    (t5543, t5555, t5558, t5561, t5564, t5567, t5571, t5572)
}
