//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2541/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2541<F: Float>(t18937: F, t4919: F, t18913: F, t16012: F, t18904: F, t18926: F, t4915: F, t18930: F, t1062: F, t6317: F, t3154: F, t4866: F) -> (F, F, F, F, F, F, F) {
    let t19951 = t4919 * t18937;
    let t19954 = t4919 * t18913;
    let t19957 = t16012 * t18904;
    let t19960 = t4915 * t18926;
    let t19963 = t4915 * t18930;
    let t19968 = t6317 * t1062;
    let t19971 = t3154 * t4866;
    (t19951, t19954, t19957, t19960, t19963, t19968, t19971)
}
