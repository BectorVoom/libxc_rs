//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 626/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk626(t1971: f64, t8842: f64, t7230: f64, t2320: f64, t7717: f64, t1685: f64, t71: f64, t131: f64, t638: f64, t639: f64, t2338: f64, t356: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8843 = t1971 * t8842;
    let t8844 = t7230 * t8843;
    let t8846 = t7717 * t2320;
    let t8849 = t71 * t1685;
    let t8850 = t8849 * t131;
    let t8852 = t638 * t639 * t8850;
    let t8854 = t2338 * t356;
    (t8843, t8844, t8846, t8849, t8850, t8852, t8854)
}
