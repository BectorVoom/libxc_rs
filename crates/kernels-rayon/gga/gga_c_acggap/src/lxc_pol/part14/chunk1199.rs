//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1199/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1199(t2001: f64, t6211: f64, t2118: f64, t6215: f64, t1998: f64, t6194: f64, t5878: f64, t36332: f64, t37957: f64, t37961: f64, t40490: f64, t40493: f64, t40497: f64, t40501: f64, t40505: f64, t40507: f64, t40511: f64, t40515: f64, t40517: f64, t40519: f64, t40521: f64) -> f64 {
    let t40523 = t2001 * t6211;
    let t40525 = t2118 * t6215;
    let t40527 = t1998 * t6194;
    let t40529 = t2001 * t5878;
    let t40531 = 0.15724046144802076034e-3_f64 * t40490 + 0.12862205435420921092e-2_f64 * t40493 + 0.21437009059034868486e-3_f64 * t40497 + 0.10718504529517434243e-2_f64 * t40501 + 0.64311027177104605458e-3_f64 * t40505 - 0.12004725073059526352e-1_f64 * t40507 - 0.18868855373762491241e-2_f64 * t40511 + 0.62896184579208304136e-3_f64 * t40515 - 0.68598428988911579156e-2_f64 * t40517 - 0.68598428988911579156e-2_f64 * t40519 - 0.34299214494455789578e-2_f64 * t40521 + 0.34299214494455789578e-2_f64 * t40523 - t37957 - 0.85748036236139473944e-3_f64 * t40525 - t36332 - 0.85748036236139473944e-3_f64 * t40527 + t37961 - 0.34299214494455789578e-2_f64 * t40529;
    t40531
}
