//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 485/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk485<F: Float>(t389: F, t4000: F, t1301: F, t1305: F, t172: F, t301: F, t342: F, t142: F, t416: F, t1163: F, t1224: F, t10: F, t1337: F, t3575: F, t1225: F, t3579: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4001 = t389 * t4000;
    let t4004 = t1301 * t1305;
    let t4007 = t342 * t172 * t301;
    let t4008 = 0.23744444444444444444e-1 * t4007;
    let t4009 = t142 * t416;
    let t4011 = t1224 * t4009 * t1163;
    let t4013 = t10 * t1337;
    let t4015 = t1224 * t4013 * t3575;
    let t4018 = t1224 * t1225 * t3579;
    (t4001, t4004, t4007, t4008, t4009, t4011, t4013, t4015, t4018)
}
