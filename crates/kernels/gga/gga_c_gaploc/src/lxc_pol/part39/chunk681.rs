//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 681/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk681<F: Float>(t10340: F, t874: F, t1445: F, t1562: F, t2854: F, t3116: F, t1645: F, t3133: F, t8352: F, t2778: F, t574: F, t12452: F, t12456: F, t12508: F, t12510: F, t12512: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12914 = t10340 * t874;
    let t12915 = t1445 * t12914;
    let t12916 = t1562 * t12915;
    let t12918 = t2854 * t3116;
    let t12919 = t1445 * t12918;
    let t12921 = 0.69017266717057349418e1 * t1562 * t12919;
    let t12922 = t1645 * t3133;
    let t12924 = 0.42900587942220512003e1 * t8352 * t12922;
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = 0.46011511144704899612e1 * t574 * t12926;
    let t12931 = 0.89376224879626066674e-1 * t12452;
    let t12932 = 0.59584149919750711116e-1 * t12456;
    let t12935 = 0.29792074959875355558e-1 * t12508;
    let t12936 = 0.29792074959875355558e-1 * t12510;
    let t12937 = 0.29792074959875355558e-1 * t12512;
    (t12914, t12915, t12916, t12918, t12919, t12921, t12922, t12924, t12925, t12926, t12928, t12931, t12932, t12935, t12936, t12937)
}
