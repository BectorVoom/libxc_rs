//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 827/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk827<F: Float>(t1991: F, t47130: F, t590: F, t739: F, t2610: F, t38907: F, t2033: F, t2365: F, t13870: F, t296: F, t1: F, t787: F, t2028: F, t325: F, t550: F, t549: F) -> (F, F, F, F, F, F, F, F) {
    let t47174 = t1991 * t739 * t47130 * t590;
    let t47178 = t2610 * t38907;
    let t47180 = t2033 * t2365 * t47178;
    let t47182 = t296 * t13870;
    let t47184 = t787 * t47182 * t1;
    let t47186 = 0.39722766613167140743e-1 * t47184 * t2028;
    let t47187 = t325 * t13870;
    let t47188 = t550 * t47187;
    let t47191 = 0.39722766613167140743e-1 * t2033 * t549 * t47188;
    (t47174, t47178, t47180, t47182, t47186, t47187, t47188, t47191)
}
