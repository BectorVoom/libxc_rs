//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1059/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1059<F: Float>(t13287: F, t2302: F, t31443: F, t8402: F, t2001: F, t5956: F, t5961: F, t6205: F, t6211: F, t2118: F, t6215: F, t1998: F, t6194: F, t5878: F, t36332: F, t37957: F, t37961: F, t40490: F, t40493: F, t40497: F, t40501: F, t40505: F, t40507: F, t40511: F) -> (F,) {
    let t40515 = t31443 * t13287 * t2302 * t8402;
    let t40517 = t2001 * t5956;
    let t40519 = t2001 * t5961;
    let t40521 = t2001 * t6205;
    let t40523 = t2001 * t6211;
    let t40525 = t2118 * t6215;
    let t40527 = t1998 * t6194;
    let t40529 = t2001 * t5878;
    let t40531 = 0.15724046144802076034e-3 * t40490 + 0.12862205435420921092e-2 * t40493 + 0.21437009059034868486e-3 * t40497 + 0.10718504529517434243e-2 * t40501 + 0.64311027177104605458e-3 * t40505 - 0.12004725073059526352e-1 * t40507 - 0.18868855373762491241e-2 * t40511 + 0.62896184579208304136e-3 * t40515 - 0.68598428988911579156e-2 * t40517 - 0.68598428988911579156e-2 * t40519 - 0.34299214494455789578e-2 * t40521 + 0.34299214494455789578e-2 * t40523 - t37957 - 0.85748036236139473944e-3 * t40525 - t36332 - 0.85748036236139473944e-3 * t40527 + t37961 - 0.34299214494455789578e-2 * t40529;
    (t40531,)
}
