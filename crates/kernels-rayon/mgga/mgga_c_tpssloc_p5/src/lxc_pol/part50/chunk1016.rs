//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 50 (v4rho2sigma2_6) CSE chunk 1016/1294 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part50_v4rho2sigma2_6_chunk1016(t26431: f64, t26470: f64, t1378: f64, t7696: f64, t794: f64, t6897: f64, t225: f64, t7704: f64, t1385: f64, t7749: f64, t3887: f64, t1375: f64, t1386: f64, t16022: f64, t16030: f64, t1843: f64, t2016: f64, t22670: f64, t22676: f64, t26366: f64, t26371: f64, t3758: f64, t3882: f64, t5326: f64, t6958: f64, t7750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t26471 = t26431 + t26470;
    let t26472 = t1378 * t26471;
    let t26474 = t794 * t7696;
    let t26475 = t6897 * t26474;
    let t26477 = t7704 * t225;
    let t26481 = t7749 * t1385;
    let t26482 = t3887 * t26481;
    let t26485 = -t26366 * t1386 + 2.0_f64 * t6958 * t5326 + 2.0_f64 * t1375 * t26371 - t3882 * t7750 - t22670 * t1843 - t16030 * t2016 - t16022 * t2016 - t1375 * t26472 - 0.41123351671205660912e-2_f64 * t26475 - t26477 * t1386 + 0.41123351671205660912e-2_f64 * t22676 - t3758 * t7750 + 2.0_f64 * t1375 * t26482;
    (t26471, t26472, t26477, t26481, t26482, t26485)
}
