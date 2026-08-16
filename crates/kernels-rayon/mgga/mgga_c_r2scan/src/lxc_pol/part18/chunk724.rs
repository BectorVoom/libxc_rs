//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 724/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk724(t1419: f64, t726: f64, t1416: f64, t2035: f64, t424: f64, t41: f64, t236: f64, t4715: f64, t735: f64, t1422: f64, t661: f64, t230: f64, t4911: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t5885 = t1419 * t726;
    let t5889 = 60.0_f64 * t1416 * t726;
    let t5890 = t424 * t2035;
    let t5891 = t41 * t5890;
    let t5893 = t4715 * t236;
    let t5895 = 0.16867793133802706421e-1_f64 * t735 * t5893;
    let t5896 = t1422 * t726;
    let t5898 = t1422 * t661;
    let t5901 = 24.0_f64 * t4911 * t230;
    (t5885, t5889, t5891, t5895, t5896, t5898, t5901)
}
