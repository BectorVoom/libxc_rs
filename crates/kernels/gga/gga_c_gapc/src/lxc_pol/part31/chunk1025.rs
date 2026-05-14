//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1025/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1025<F: Float>(t10031: F, t33748: F, t3402: F, t1084: F, t9923: F, t10043: F, t11945: F, t11387: F, t3363: F, t1089: F, t29228: F, t3784: F, t11944: F, t2200: F, t9896: F, t18856: F, t2767: F, t3717: F) -> (F, F, F, F, F, F, F, F) {
    let t33750 = t3402 * t33748 * t10031;
    let t33753 = t1084 * t33748 * t9923;
    let t33755 = t10043 * t11945;
    let t33757 = t3363 * t11387;
    let t33758 = t33757 * t1089;
    let t33760 = t3784 * t29228;
    let t33763 = t11944 * t2200 * t9896;
    let t33766 = t18856 * t3717 * t2767;
    (t33750, t33753, t33755, t33757, t33758, t33760, t33763, t33766)
}
