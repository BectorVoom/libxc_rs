//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 667/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk667(t11894: f64, t723: f64, t1445: f64, t2925: f64, t3009: f64, t10015: f64, t11055: f64, t11869: f64, t11875: f64, t11878: f64, t11881: f64, t11884: f64, t11887: f64, t11891: f64, t1998: f64, t2004: f64, t2087: f64, t2103: f64, t2639: f64, t3651: f64, t807: f64, t813: f64, t833: f64) -> f64 {
    let t11895 = t11894 * t723;
    let t11896 = t1445 * t11895;
    let t11899 = t3009 * t2925;
    let t11900 = t1445 * t11899;
    let t11904 = 0.63904876589867916126e-1_f64 * t10015 + 0.43710935587469654631e2_f64 * t833 * t11869 - 0.25025342966295298669e1_f64 * t3651 * t2639 - 0.92023022289409799224e1_f64 * t813 * t11875 - 0.11502877786176224903e2_f64 * t1998 * t11878 + 0.23005755572352449806e2_f64 * t833 * t11881 + 0.14300195980740170668e1_f64 * t2103 * t11884 + 0.35750489951850426669e0_f64 * t2004 * t11887 + 0.46011511144704899612e1_f64 * t807 * t11891 - 0.69017266717057349418e1_f64 * t2087 * t11896 - 0.13803453343411469884e2_f64 * t2087 * t11900 - 0.23005755572352449806e1_f64 * t11055;
    t11904
}
