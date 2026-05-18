//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 471/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk471<F: Float>(t397: F, t3979: F, t403: F, t396: F, t172: F, t301: F, t342: F, t142: F, t416: F, t10: F, t1337: F, t1232: F, t357: F) -> (F, F, F, F, F, F, F) {
    let t3981 = t397 * t3979 * t403;
    let t3983 = F::new(0.11993859144118211475e-1) * t396 * t3981;
    let t4007 = t342 * t172 * t301;
    let t4008 = F::new(0.23744444444444444444e-1) * t4007;
    let t4009 = t142 * t416;
    let t4013 = t10 * t1337;
    let t4029 = t1232 * t357;
    (t3981, t3983, t4007, t4008, t4009, t4013, t4029)
}
