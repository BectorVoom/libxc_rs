//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 950/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk950<F: Float>(t1406: F, t1828: F, t5218: F, t5219: F, t108: F, t1878: F, t267: F, t5221: F, t17591: F, t17596: F, t17601: F, t17606: F, t17608: F, t17610: F, t17613: F, t17617: F, t17621: F) -> (F, F, F) {
    let t17625 = F::new(32.0) / F::new(15.0) * t5218 * t5219 * t1406 * t1828;
    let t17627 = t1878 * t108 * t267;
    let t17629 = F::new(64.0) / F::new(15.0) * t17627 * t5221;
    let t17630 = t17591 + t17596 - t17601 - t17606 - t17608 + t17610 - t17613 - t17617 + t17621 - t17625 - t17629;
    (t17625, t17629, t17630)
}
