//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1143/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1143<F: Float>(t1882: F, t28252: F, t1456: F, t2492: F, t28357: F, t8392: F, t28322: F, t28326: F, t28398: F, t28226: F, t9802: F, t258: F, t27742: F, t28218: F, t28130: F, t10051: F, t1424: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t110447 = 2.0 / 9.0 * t1882 * t28252;
    let t110478 = t2492 * t1456;
    let t110489 = 2.0 / 27.0 * t8392 * t28357;
    let t110496 = 4.0 / 9.0 * t1882 * t28322;
    let t110498 = 4.0 / 9.0 * t1882 * t28326;
    let t110503 = 2.0 / 27.0 * t8392 * t28398;
    let t110517 = 2.0 / 27.0 * t1882 * t28226;
    let t110539 = t9802 * t1456;
    let t110543 = t258 * t27742;
    let t110559 = 4.0 / 9.0 * t1882 * t28218;
    let t110575 = 4.0 / 9.0 * t8392 * t28130;
    let t110576 = t10051 * t1424;
    (t110447, t110478, t110489, t110496, t110498, t110503, t110517, t110539, t110543, t110559, t110575, t110576)
}
