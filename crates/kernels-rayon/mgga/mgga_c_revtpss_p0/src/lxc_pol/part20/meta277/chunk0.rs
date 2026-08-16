//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1133/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1133(t1086: f64, t3046: f64, t3090: f64, t1043: f64, t3075: f64, t1045: f64, t3117: f64, t3316: f64, t994: f64, t4891: f64, t11659: f64, t4910: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11865 = t3046 * t1086;
    let t11866 = t11865 * t3090;
    let t11869 = t3075 * t1043;
    let t11870 = t11869 * t1045;
    let t11871 = t3117 * t11870;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    let t11876 = t11659 * t4910;
    (t11865, t11866, t11869, t11870, t11871, t11874, t11875, t11876)
}
