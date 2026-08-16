//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 53 (v4rho2sigma2_9) CSE chunk 623/1059 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part53_v4rho2sigma2_9_chunk623(t240: f64, t6950: f64, t1336: f64, t1369: f64, t2010: f64, t6883: f64, t552: f64, t562: f64, t1307: f64, t6637: f64, t6888: f64, t2009: f64, t794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t6951 = t6950 * t240;
    let t6952 = t1336 * t6951;
    let t6953 = t6952 * t1369;
    let t6966 = t6883 * t2010;
    let t6968 = t552 * t562;
    let t6969 = t6968 * t1307;
    let t6970 = t6637 * t6969;
    let t6971 = t6888 * t6970;
    let t6973 = t794 * t2009;
    (t6951, t6952, t6953, t6966, t6968, t6969, t6970, t6971, t6973)
}
