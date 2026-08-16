//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 481/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk481(t13875: f64, t1986: f64, t3141: f64, t334: f64, t797: f64, t305: f64, t353: f64, t3122: f64, t4179: f64, t7: f64, t2003: f64, t3133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13876 = t1986 * t13875;
    let t13877 = t3141 * t13876;
    let t13879 = t797 * t334;
    let t13880 = t1986 * t13879;
    let t13881 = t3141 * t13880;
    let t13883 = t305 * t353;
    let t13884 = t1986 * t13883;
    let t13885 = t3141 * t13884;
    let t13888 = t3122 * t7 * t4179;
    let t13889 = t13888 * t2003;
    let t13890 = t3133 * t13889;
    (t13876, t13877, t13880, t13881, t13884, t13885, t13888, t13889, t13890)
}
