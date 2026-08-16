//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 705/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk705<F: Float>(t120: F, t8327: F, t1134: F, t991: F, t2894: F, t385: F, t4059: F, t522: F, t1006: F, t1448: F, t1464: F, t2953: F) -> (F, F, F, F, F) {
    let t8328 = t120 * t8327;
    let t8330 = t1134 * t991;
    let t8332 = t385 * t2894;
    let t8334 = t4059 * t522;
    let t8335 = t1006 * t8334;
    let t8337 = t1448 * t1464;
    let t8338 = t2953 * t8337;
    (t8328, t8330, t8332, t8335, t8338)
}
