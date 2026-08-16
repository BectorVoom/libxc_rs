//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1200/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1200(t32697: f64, t6889: f64, t1985: f64, t1799: f64, t31099: f64, t22635: f64, t22633: f64, t1998: f64, t59: f64, t6926: f64, t1825: f64, t6943: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t32698 = t6889 * t32697;
    let t32700 = 0.16449340668482264365e-1_f64 * t1985 * t32698;
    let t32704 = t31099 * t1799;
    let t32705 = t22635 * t32704;
    let t32707 = 0.3289868133696452873e-1_f64 * t22633 * t32705;
    let t32711 = t1998 * t59 * t1799;
    let t32712 = t6926 * t32711;
    let t32714 = t6943 * t1825;
    (t32698, t32700, t32704, t32705, t32707, t32711, t32712, t32714)
}
