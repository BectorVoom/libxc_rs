//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 569/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk569<F: Float>(t1445: F, t2950: F, t2958: F, t701: F, t1035: F, t773: F, t1: F, t1022: F, t106: F) -> (F, F, F, F, F, F) {
    let t3028 = t1445 * t2950;
    let t3031 = t2958 * t701;
    let t3032 = t1445 * t3031;
    let t3035 = t773 * t1035;
    let t3038 = t1022 * t1;
    let t3039 = t3038 * t106;
    (t3028, t3031, t3032, t3035, t3038, t3039)
}
