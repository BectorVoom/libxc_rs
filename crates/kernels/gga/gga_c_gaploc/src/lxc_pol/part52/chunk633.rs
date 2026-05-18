//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 633/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk633<F: Float>(t11894: F, t723: F, t1445: F, t2925: F, t3009: F, t10015: F, t11055: F, t11869: F, t11875: F, t11878: F, t11881: F, t11884: F, t11887: F, t11891: F, t1998: F, t2004: F, t2087: F, t2103: F, t2639: F, t3651: F, t807: F, t813: F, t833: F) -> F {
    let t11895 = t11894 * t723;
    let t11896 = t1445 * t11895;
    let t11899 = t3009 * t2925;
    let t11900 = t1445 * t11899;
    let t11904 = F::new(0.63904876589867916126e-1) * t10015 + F::new(0.43710935587469654631e2) * t833 * t11869 - F::new(0.25025342966295298669e1) * t3651 * t2639 - F::new(0.92023022289409799224e1) * t813 * t11875 - F::new(0.11502877786176224903e2) * t1998 * t11878 + F::new(0.23005755572352449806e2) * t833 * t11881 + F::new(0.14300195980740170668e1) * t2103 * t11884 + F::new(0.35750489951850426669e0) * t2004 * t11887 + F::new(0.46011511144704899612e1) * t807 * t11891 - F::new(0.69017266717057349418e1) * t2087 * t11896 - F::new(0.13803453343411469884e2) * t2087 * t11900 - F::new(0.23005755572352449806e1) * t11055;
    t11904
}
