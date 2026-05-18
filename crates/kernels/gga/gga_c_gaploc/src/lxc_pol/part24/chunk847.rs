//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 847/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk847<F: Float>(t549: F, t7981: F, t1397: F, t2897: F, t1402: F, t2783: F, t1359: F, t986: F, t1415: F, t107: F, t7887: F, t544: F) -> (F, F, F, F, F, F, F) {
    let t8226 = t549 * t7981;
    let t8229 = t1397 * t2897;
    let t8233 = t1402 * t2783;
    let t8237 = t1359 * t986;
    let t8238 = t1415 * t8237;
    let t8247 = t7887 * t107;
    let t8248 = t544 * t8247;
    (t8226, t8229, t8233, t8237, t8238, t8247, t8248)
}
