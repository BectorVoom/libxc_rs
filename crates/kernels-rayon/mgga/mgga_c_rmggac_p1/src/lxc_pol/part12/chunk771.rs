//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 771/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk771(t333: f64, t830: f64, t262: f64, t2073: f64, t22: f64, t4616: f64, t326: f64, t265: f64, t7835: f64, t876: f64, t2078: f64, t26: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35924 = t830 * t333;
    let t35925 = t262 * t35924;
    let t35926 = t2073 * t35925;
    let t35928 = t4616 * t22;
    let t35929 = t326 * t35928;
    let t35937 = t7835 * t262 * t265 * t876;
    let t35959 = t2078 * t26;
    (t35924, t35925, t35926, t35929, t35937, t35959)
}
