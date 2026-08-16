//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 683/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk683(t515: f64, t9843: f64, t1971: f64, t7230: f64, t2310: f64, t8571: f64, t2320: f64, t9222: f64, t1763: f64, t7703: f64, t1356: f64, t1737: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9844 = t515 * t9843;
    let t9845 = t1971 * t9844;
    let t9846 = t7230 * t9845;
    let t9847 = 0.1064114997332445985e-4_f64 * t9846;
    let t9848 = t8571 * t2310;
    let t9849 = 0.85129199786595678796e-5_f64 * t9848;
    let t9850 = t9222 * t2320;
    let t9851 = 0.1064114997332445985e-4_f64 * t9850;
    let t9852 = t7703 * t1763;
    let t9853 = t1356 * t9852;
    let t9854 = 0.11974241701863808564e0_f64 * t9853;
    let t9855 = t665 * t1737;
    (t9845, t9847, t9849, t9851, t9852, t9854, t9855)
}
