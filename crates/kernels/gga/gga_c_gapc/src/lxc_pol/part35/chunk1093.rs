//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1093/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1093<F: Float>(t11216: F, t13646: F, t520: F, t13654: F, t35541: F, t3948: F, t11258: F, t2932: F, t3946: F, t1006: F, t3639: F, t4026: F, t35568: F, t583: F, t8524: F, t3635: F, t8422: F) -> (F, F, F, F, F, F) {
    let t35650 = t11216 * t520 * t13646;
    let t35653 = t35541 * t3948 * t13654;
    let t35656 = t2932 * t3946 * t11258;
    let t35659 = t1006 * t3639 * t4026;
    let t35662 = t8524 * t35568 * t583;
    let t35664 = t8422 * t3635;
    (t35650, t35653, t35656, t35659, t35662, t35664)
}
