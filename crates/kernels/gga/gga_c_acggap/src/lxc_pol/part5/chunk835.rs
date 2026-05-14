//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 835/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk835<F: Float>(t968: F, t981: F, t974: F, t177: F, t3646: F, t414: F, t973: F, t980: F, t3559: F, t377: F, t107: F, t118: F, t11805: F, t11820: F, t4: F, t150: F, t164: F) -> (F, F, F, F, F, F, F) {
    let t13308 = t981 * t968;
    let t13310 = t974 * t968;
    let t13314 = 0.40015750243531754508e-2 * t3646 * t414 * t177;
    let t13317 = 0.34013387707001991332e-1 * t980 * t973 * t177;
    let t13320 = 0.15117061203111996148e0 * t377 * t3559 * t177;
    let t13326 = (0.43209876543209876543e0 * t4 * t11805 * t107 + 0.15432407407407407408e0 * t11820) * t118;
    let t13330 = 0.21437009059034868486e-3 * t13326 * t150 * t164 * t177;
    (t13308, t13310, t13314, t13317, t13320, t13326, t13330)
}
