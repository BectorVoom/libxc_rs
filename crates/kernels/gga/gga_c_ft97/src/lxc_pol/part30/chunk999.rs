//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 999/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk999<F: Float>(t152712: F, t152765: F, t152831: F, t152895: F, t152945: F, t153386: F, t153432: F, t153470: F, t1466: F, t35798: F, t681: F, t112384: F, t142602: F, t142611: F, t142613: F, t152648: F, t152651: F, t193: F, t28966: F, t28968: F, t29002: F, t3051: F, t312: F, t33966: F, t36105: F, t6210: F, t6222: F, t7580: F, t7581: F) -> (F, F) {
    let t153473 = t152712 + t152765 + t152831 + t152895 + t152945 + t153386 + t153432 + t153470;
    let t153486 = t1466 * t681 * t35798;
    let t153492 = 4.0 * t152648 + t152651 / 9.0 + t7580 * t3051 * t29002 / 9.0 + t142602 + 2.0 * t153473 * t312 - t142611 / 3.0 + t6210 * t36105 / 6.0 - t142613 / 18.0 - t7581 * t28968 / 3.0 + t1466 * t193 * t33966 * t28966 - t153486 / 3.0 - 2.0 / 3.0 * t1466 * t193 * t6222 * t112384;
    (t153473, t153492)
}
