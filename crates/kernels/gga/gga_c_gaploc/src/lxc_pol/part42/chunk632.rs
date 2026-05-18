//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 632/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk632<F: Float>(t11627: F, t723: F, t1445: F, t2925: F, t2949: F, t11628: F, t11632: F, t1457: F, t11623: F, t11724: F, t701: F, t2089: F, t3614: F) -> (F, F, F, F, F, F, F, F) {
    let t11868 = t11627 * t723;
    let t11869 = t1445 * t11868;
    let t11874 = t2949 * t2925;
    let t11875 = t1445 * t11874;
    let t11878 = t1445 * t11628;
    let t11881 = t1445 * t11632;
    let t11884 = t1457 * t11632;
    let t11887 = t1457 * t11623;
    let t11890 = t11724 * t701;
    let t11891 = t1445 * t11890;
    let t11894 = t2089 * t3614;
    (t11869, t11875, t11878, t11881, t11884, t11887, t11891, t11894)
}
