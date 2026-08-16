//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 967/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk967(t1992: f64, t22635: f64, t26225: f64, t6439: f64, t1985: f64, t28186: f64, t6889: f64, t6906: f64, t120217: f64, t120220: f64, t22633: f64, t32704: f64, t90566: f64) -> (f64, f64, f64, f64, f64) {
    let t127197 = 0.9869604401089358619e-1_f64 * t1992 * t22635 * t26225 * t6439;
    let t127201 = 0.16449340668482264365e-1_f64 * t1985 * t6889 * t6906 * t28186;
    let t127202 = 0.3289868133696452873e-1_f64 * t120217;
    let t127203 = 0.3289868133696452873e-1_f64 * t120220;
    let t127210 = 0.6579736267392905746e-1_f64 * t22633 * t90566 * t32704;
    (t127197, t127201, t127202, t127203, t127210)
}
