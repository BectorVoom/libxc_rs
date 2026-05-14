//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 627/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk627<F: Float>(t2778: F, t3116: F, t1445: F, t574: F, t12446: F, t12450: F, t12452: F, t12456: F, t12906: F, t12909: F, t12911: F, t12912: F, t12916: F, t12921: F, t12924: F, t12508: F) -> (F, F, F, F) {
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = 0.46011511144704899612e1 * t574 * t12926;
    let t12929 = 0.63904876589867916127e-1 * t12446;
    let t12930 = 0.63904876589867916127e-1 * t12450;
    let t12931 = 0.89376224879626066674e-1 * t12452;
    let t12932 = 0.59584149919750711116e-1 * t12456;
    let t12933 = -0.92023022289409799224e1 * t12906 + t12909 + t12911 + 0.71500979903700853338e0 * t12912 - 0.13803453343411469884e2 * t12916 - t12921 + t12924 - t12928 - t12929 + t12930 - t12931 + t12932;
    let t12935 = 0.29792074959875355558e-1 * t12508;
    (t12925, t12926, t12933, t12935)
}
