//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 761/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk761<F: Float>(t1890: F, t2530: F, t590: F, t1457: F, t7250: F, t7254: F, t1: F, t106: F, t316: F, t2154: F, t774: F, t959: F, t1865: F, t7290: F, t2365: F, t6111: F) -> (F, F, F, F, F, F, F) {
    let t7696 = t1890 * t2530;
    let t7697 = t7696 * t590;
    let t7700 = t1457 * t7250;
    let t7703 = t1457 * t7254;
    let t7710 = t2530 * t1;
    let t7711 = t7710 * t106;
    let t7712 = t7711 * t316;
    let t7715 = t2154 * t774;
    let t7716 = t7715 * t959;
    let t7718 = t7290 * t1865;
    let t7719 = t2365 * t7718;
    let t7720 = t6111 * t7719;
    (t7696, t7697, t7700, t7703, t7712, t7716, t7720)
}
