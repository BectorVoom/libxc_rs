//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1271/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1271(t6889: f64, t7700: f64, t1985: f64, t1807: f64, t2006: f64, t1811: f64, t6916: f64, t1799: f64, t236: f64) -> (f64, f64, f64, f64, f64) {
    let t7701 = t6889 * t7700;
    let t7702 = t1985 * t7701;
    let t7704 = t1807 * t2006;
    let t7706 = t6916 * t1811;
    let t7708 = t236 * t1799;
    (t7701, t7702, t7704, t7706, t7708)
}
