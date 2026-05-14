//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1351/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1351<F: Float>(t2168: F, t35843: F, t3922: F, t6204: F, t32069: F, t4158: F, t32096: F, t33593: F, t1411: F, t32000: F, t33600: F, t20160: F, t33438: F, t9426: F, t12827: F, t19048: F, t9461: F) -> (F, F, F, F, F, F, F) {
    let t113690 = t6204 * t35843 * t2168 * t3922;
    let t113695 = t6204 * t32069 * t2168 * t4158;
    let t113702 = 0.69444444444444444446e-2 * t32096 * t33593;
    let t113704 = t1411 * t32000 * t33600;
    let t113708 = t20160 * t33438;
    let t113710 = 0.26805555555555555556e-2 * t9426 * t113708;
    let t113714 = t12827 * t9461 * t19048;
    (t113690, t113695, t113702, t113704, t113708, t113710, t113714)
}
