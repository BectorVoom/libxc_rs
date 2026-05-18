//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 697/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk697<F: Float>(t12918: F, t1445: F, t1562: F, t1645: F, t3133: F, t8352: F, t2778: F, t3116: F, t574: F, t12446: F, t12450: F, t12452: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12919 = t1445 * t12918;
    let t12921 = F::new(0.69017266717057349418e1) * t1562 * t12919;
    let t12922 = t1645 * t3133;
    let t12924 = F::new(0.42900587942220512003e1) * t8352 * t12922;
    let t12925 = t2778 * t3116;
    let t12926 = t1445 * t12925;
    let t12928 = F::new(0.46011511144704899612e1) * t574 * t12926;
    let t12929 = F::new(0.63904876589867916127e-1) * t12446;
    let t12930 = F::new(0.63904876589867916127e-1) * t12450;
    let t12931 = F::new(0.89376224879626066674e-1) * t12452;
    (t12919, t12921, t12922, t12924, t12925, t12926, t12928, t12929, t12930, t12931)
}
