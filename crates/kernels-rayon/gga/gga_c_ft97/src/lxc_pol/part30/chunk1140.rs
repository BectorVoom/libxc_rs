//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1140/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1140(t152712: f64, t152765: f64, t152831: f64, t152895: f64, t152945: f64, t153386: f64, t153432: f64, t153470: f64, t1466: f64, t35798: f64, t681: f64, t112384: f64, t142602: f64, t142611: f64, t142613: f64, t152648: f64, t152651: f64, t193: f64, t28966: f64, t28968: f64, t29002: f64, t3051: f64, t312: f64, t33966: f64, t36105: f64, t6210: f64, t6222: f64, t7580: f64, t7581: f64) -> (f64, f64) {
    let t153473 = t152712 + t152765 + t152831 + t152895 + t152945 + t153386 + t153432 + t153470;
    let t153486 = t1466 * t681 * t35798;
    let t153492 = 4.0_f64 * t152648 + t152651 / 9.0_f64 + t7580 * t3051 * t29002 / 9.0_f64 + t142602 + 2.0_f64 * t153473 * t312 - t142611 / 3.0_f64 + t6210 * t36105 / 6.0_f64 - t142613 / 18.0_f64 - t7581 * t28968 / 3.0_f64 + t1466 * t193 * t33966 * t28966 - t153486 / 3.0_f64 - 2.0_f64 / 3.0_f64 * t1466 * t193 * t6222 * t112384;
    (t153473, t153492)
}
