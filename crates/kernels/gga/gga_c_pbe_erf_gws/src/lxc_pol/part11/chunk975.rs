//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 975/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk975<F: Float>(t1673: F, t3399: F, t11190: F, t2007: F, t1697: F, t3562: F, t17197: F, t3522: F, t639: F, t1672: F, t211: F, t3391: F) -> (F, F, F, F, F) {
    let t32093 = t3399 * t1673;
    let t32097 = t11190 * t2007;
    let t32114 = t3562 * t1697;
    let t32202 = t639 * t17197 * t3522;
    let t32215 = t211 * t1672 * t3391;
    (t32093, t32097, t32114, t32202, t32215)
}
