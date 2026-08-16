//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 485/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk485(t1495: f64, t2011: f64, t1395: f64, t1464: f64, t1364: f64, t1391: f64, t1924: f64, t1944: f64, t1949: f64, t1985: f64, t2004: f64, t2008: f64, t507: f64) -> (f64, f64, f64, f64) {
    let t2012 = t1495 * t2011;
    let t2013 = t1395 * t2012;
    let t2014 = t1464 * t2013;
    let t2016 = t1924 * t507 - 0.66725e-1_f64 * t1364 * t1944 + t1391 + 0.16581944444444444444e-2_f64 * t1949 + 0.24872916666666666666e-2_f64 * t1985 - 0.24872916666666666666e-2_f64 * t2004 - 0.66327777777777777776e-2_f64 * t2008 + 0.16581944444444444444e-2_f64 * t2014;
    (t2012, t2013, t2014, t2016)
}
