//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 886/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk886<F: Float>(t13716: F, t577: F, t584: F, t1072: F, t167: F, t7322: F, t145: F, t301: F, t721: F, t174: F, t372: F, t7859: F) -> (F, F, F, F, F) {
    let t30594 = t13716 * t577;
    let t30595 = t30594 * t584;
    let t30596 = F::new(0.37042881944444444445e0) * t30595;
    let t30598 = t7322 * t167 * t1072;
    let t30601 = t30598 * t145 * t301 * t721;
    let t30605 = t7859 * t174 * t372 * t721;
    (t30594, t30596, t30598, t30601, t30605)
}
