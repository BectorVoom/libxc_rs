//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1225/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1225<F: Float>(t32540: F, t34547: F, t34549: F, t34561: F, t34566: F, t34575: F, t34578: F, t34582: F, t34586: F, t34590: F, t37150: F, t37158: F, t37167: F, t39382: F, t39386: F, t39389: F, t39391: F, t39393: F) -> F {
    let t41623 = -F::new(0.68598428988911579156e-2) * t34547 - F::new(0.32012600194825403606e-1) * t34549 + t37150 + F::new(0.31448092289604152068e-2) * t39382 - F::new(0.47172138434406228102e-2) * t39386 + F::new(0.62896184579208304138e-3) * t39389 + F::new(0.37737710747524982484e-2) * t34561 + F::new(0.27439371595564631662e-1) * t39391 + F::new(0.12862205435420921092e-2) * t39393 - t34566 + t37158 - t34575 - t32540 + F::new(0.12579236915841660827e-1) * t34578 - F::new(0.5031694766336664331e-2) * t34582 + F::new(0.75475421495049964968e-2) * t34586 - F::new(0.34299214494455789578e-2) * t34590 - t37167;
    t41623
}
