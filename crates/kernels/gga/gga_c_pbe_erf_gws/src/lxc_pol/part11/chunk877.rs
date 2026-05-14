//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 877/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk877<F: Float>(t1251: F, t3440: F, t1243: F, t3422: F, t1033: F, t7844: F, t10418: F, t586: F, t1673: F, t3399: F, t11190: F, t2007: F, t1697: F, t3562: F, t17197: F, t3522: F, t639: F) -> (F, F, F, F, F, F, F, F) {
    let t31803 = t1251 * t3440;
    let t31805 = t1243 * t3422;
    let t31879 = t1033 * t7844;
    let t32019 = t10418 * t586;
    let t32093 = t3399 * t1673;
    let t32097 = t11190 * t2007;
    let t32114 = t3562 * t1697;
    let t32202 = t639 * t17197 * t3522;
    (t31803, t31805, t31879, t32019, t32093, t32097, t32114, t32202)
}
