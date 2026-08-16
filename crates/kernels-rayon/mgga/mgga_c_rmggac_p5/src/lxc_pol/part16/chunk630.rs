//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 630/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk630(t265: f64, t570: f64, t2079: f64, t262: f64, t2068: f64, t8705: f64, t2073: f64, t8701: f64, t1652: f64, t36: f64, t118: f64, t4616: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8915 = t265 * t570;
    let t8917 = t2079 * t262 * t8915;
    let t8919 = t2068 * t8705;
    let t8921 = t2073 * t8701;
    let t8924 = t36 * t1652;
    let t8926 = t2079 * t262 * t8924;
    let t8940 = t118 * t4616;
    (t8915, t8917, t8919, t8921, t8926, t8940)
}
