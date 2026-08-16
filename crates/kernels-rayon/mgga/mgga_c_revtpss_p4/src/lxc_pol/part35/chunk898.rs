//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 898/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk898(t23019: f64, t23041: f64, t1427: f64, t10157: f64, t14091: f64, t14097: f64, t14105: f64, t1424: f64, t14280: f64, t14290: f64, t14294: f64, t14297: f64, t1904: f64, t22390: f64, t22428: f64, t22447: f64, t22450: f64, t22454: f64, t5715: f64, t6919: f64) -> (f64, f64, f64) {
    let t23042 = t23019 + t23041;
    let t23043 = t1427 * t23042;
    let t23058 = 0.39029762157531132076e-1_f64 * t14091 + 0.21951497276451705329e-1_f64 * t14097 - 0.34697458558045176417e-2_f64 * t14105 - 0.65854491829355115987e0_f64 * t1424 * t23043 + 0.32927245914677557992e-1_f64 * t22428 - 0.19756347548806534796e1_f64 * t5715 * t6919 - t10157 - 0.39029762157531132076e-1_f64 * t14280 - 0.19756347548806534796e1_f64 * t22390 * t1904 - 0.16463622957338778996e-1_f64 * t22447 - 0.32927245914677557992e-1_f64 * t22450 + 0.58544643236296698113e-1_f64 * t22454 - 0.21951497276451705329e-1_f64 * t14290 + 0.34697458558045176417e-2_f64 * t14294 + 0.19514881078765566038e-2_f64 * t14297;
    (t23042, t23043, t23058)
}
