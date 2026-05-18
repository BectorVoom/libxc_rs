//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1058/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1058<F: Float>(t6455: F, t900: F, t1423: F, t2317: F, t501: F, t6551: F, t2530: F, t723: F) -> (F, F, F, F) {
    let t21414 = t900 * t6455;
    let t21417 = t1423 * t2317;
    let t21438 = t6551 * t501;
    let t21446 = t2530 * t723;
    (t21414, t21417, t21438, t21446)
}
