//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 661/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk661<F: Float>(t1954: F, t8361: F, t8362: F, t132: F, t2878: F, t2881: F, t2899: F, t385: F, t1510: F, t2880: F, t120: F, t3954: F, t436: F, t2941: F, t101: F, t1762: F) -> (F, F, F, F, F, F) {
    let t8364 = t8361 * t8362 * t1954;
    let t8368 = t132 * t2878;
    let t8369 = t8368 * t2881;
    let t8371 = t385 * t2899;
    let t8373 = t2880 * t1510;
    let t8374 = t120 * t8373;
    let t8376 = t436 * t3954;
    let t8377 = t2941 * t8376;
    let t8379 = t1762 * t101;
    (t8364, t8369, t8371, t8374, t8377, t8379)
}
