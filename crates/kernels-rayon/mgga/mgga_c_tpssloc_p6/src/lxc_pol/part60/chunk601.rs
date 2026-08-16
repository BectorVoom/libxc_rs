//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 601/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk601(t6889: f64, t7700: f64, t1985: f64, t1807: f64, t2006: f64, t1811: f64, t6916: f64, t1799: f64, t236: f64, t1998: f64, t6926: f64, t1339: f64, t1825: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t7701 = t6889 * t7700;
    let t7702 = t1985 * t7701;
    let t7704 = t1807 * t2006;
    let t7706 = t6916 * t1811;
    let t7708 = t236 * t1799;
    let t7709 = t1998 * t7708;
    let t7710 = t6926 * t7709;
    let t7712 = t1339 * t1825;
    (t7701, t7702, t7704, t7706, t7709, t7710, t7712)
}
