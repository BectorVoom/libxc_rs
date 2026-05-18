//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 939/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk939<F: Float>(t22817: F, t7203: F, t1669: F, t15: F, t32163: F, t5555: F, t32161: F, t22755: F, t9: F, t420: F, t5578: F, t1608: F, t32167: F, t32237: F) -> (F, F, F, F, F, F, F) {
    let t136433 = t22817 * t7203;
    let t136434 = t1669 * t136433;
    let t136457 = t5555 * t15 * t32163;
    let t136458 = t32161 * t136457;
    let t136468 = t1669 * t22755 * t9;
    let t136469 = t5578 * t420;
    let t136474 = t1608 * t32167 * t32237;
    (t136433, t136434, t136457, t136458, t136468, t136469, t136474)
}
