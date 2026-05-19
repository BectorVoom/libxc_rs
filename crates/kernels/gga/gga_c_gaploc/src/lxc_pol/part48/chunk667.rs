//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 667/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk667<F: Float>(t11894: F, t723: F, t1445: F, t2925: F, t3009: F, t10015: F, t11055: F, t11869: F, t11875: F, t11878: F, t11881: F, t11884: F, t11887: F, t11891: F, t1998: F, t2004: F, t2087: F, t2103: F, t2639: F, t3651: F, t807: F, t813: F, t833: F) -> F {
    let t11895 = t11894 * t723;
    let t11896 = t1445 * t11895;
    let t11899 = t3009 * t2925;
    let t11900 = t1445 * t11899;
    let t11904 = F::cast_from(0.63904876589867916126e-1_f64) * t10015 + F::cast_from(0.43710935587469654631e2_f64) * t833 * t11869 - F::cast_from(0.25025342966295298669e1_f64) * t3651 * t2639 - F::cast_from(0.92023022289409799224e1_f64) * t813 * t11875 - F::cast_from(0.11502877786176224903e2_f64) * t1998 * t11878 + F::cast_from(0.23005755572352449806e2_f64) * t833 * t11881 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t11884 + F::cast_from(0.35750489951850426669e0_f64) * t2004 * t11887 + F::cast_from(0.46011511144704899612e1_f64) * t807 * t11891 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t11896 - F::cast_from(0.13803453343411469884e2_f64) * t2087 * t11900 - F::cast_from(0.23005755572352449806e1_f64) * t11055;
    t11904
}
