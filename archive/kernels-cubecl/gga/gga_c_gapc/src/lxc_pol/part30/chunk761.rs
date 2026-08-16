//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 761/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk761<F: Float>(t1740: F, t9020: F, t19: F, t424: F, t3114: F, t3117: F, t3123: F, t8888: F, t1448: F, t3116: F, t3115: F, t3064: F, t3122: F) -> (F, F, F, F, F, F) {
    let t9038 = t9020 * t1740;
    let t9040 = t424 * t19;
    let t9041 = t9040 * t3114;
    let t9042 = t9041 * t3117;
    let t9044 = t8888 * t3123;
    let t9047 = t1448 * t3116;
    let t9048 = t3115 * t9047;
    let t9050 = t3064 * t3122;
    (t9038, t9041, t9042, t9044, t9048, t9050)
}
