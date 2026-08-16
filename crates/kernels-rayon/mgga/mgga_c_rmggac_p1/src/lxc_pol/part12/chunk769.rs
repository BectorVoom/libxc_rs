//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 769/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk769(t35861: f64, t265: f64, t848: f64, t262: f64, t2073: f64, t2079: f64, t866: f64, t833: f64, t2068: f64, t321: f64, t830: f64, t2067: f64, t25529: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t35862 = 0.68297526743963945143e0_f64 * t35861;
    let t35863 = t265 * t848;
    let t35864 = t262 * t35863;
    let t35865 = t2073 * t35864;
    let t35869 = t2079 * t262 * t265 * t866;
    let t35871 = t265 * t833;
    let t35872 = t262 * t35871;
    let t35873 = t2068 * t35872;
    let t35875 = t830 * t321;
    let t35876 = t262 * t35875;
    let t35877 = t2068 * t35876;
    let t35879 = t25529 * t2067;
    (t35862, t35863, t35864, t35865, t35869, t35871, t35872, t35873, t35875, t35876, t35877, t35879)
}
