//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 57 (v4rho2sigma2_13) CSE chunk 850/1049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part57_v4rho2sigma2_13_chunk850(t32678: f64, t1842: f64, t31090: f64, t22635: f64, t1992: f64, t6906: f64, t7749: f64, t6889: f64, t1985: f64, t1799: f64, t31099: f64, t22633: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32679 = 2.0_f64 * t32678;
    let t32693 = t31090 * t1842;
    let t32694 = t22635 * t32693;
    let t32696 = 0.3289868133696452873e-1_f64 * t1992 * t32694;
    let t32697 = t6906 * t7749;
    let t32698 = t6889 * t32697;
    let t32700 = 0.16449340668482264365e-1_f64 * t1985 * t32698;
    let t32704 = t31099 * t1799;
    let t32705 = t22635 * t32704;
    let t32707 = 0.3289868133696452873e-1_f64 * t22633 * t32705;
    (t32679, t32693, t32694, t32696, t32697, t32698, t32700, t32704, t32705, t32707)
}
