//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 727/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk727<F: Float>(t7284: F, t974: F, t24847: F, t1089: F, t491: F, t7327: F, t11947: F, t2157: F, t111: F, t2169: F, t192: F, t531: F) -> (F, F, F, F, F) {
    let t24848 = t974 * t7284;
    let t24849 = t24847 * t24848;
    let t24850 = t491 * t1089;
    let t24851 = t7327 * t24850;
    let t24909 = t2157 * t11947;
    let t24972 = t2169 * t111;
    let t24994 = t192 * t531;
    (t24849, t24851, t24909, t24972, t24994)
}
