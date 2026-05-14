//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 832/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk832<F: Float>(t136363: F, t22796: F, t5572: F, t173: F, t22581: F, t32146: F, t1691: F, t32318: F, t22817: F, t7203: F, t1669: F, t15: F, t32163: F, t5555: F, t32161: F, t22755: F, t9: F) -> (F, F, F, F, F, F, F, F, F) {
    let t136365 = t22796 * t136363 * t5572;
    let t136367 = t22581 * t173;
    let t136369 = t32146 * t136367 * t5572;
    let t136403 = t1691 * t32318;
    let t136433 = t22817 * t7203;
    let t136434 = t1669 * t136433;
    let t136457 = t5555 * t15 * t32163;
    let t136458 = t32161 * t136457;
    let t136468 = t1669 * t22755 * t9;
    (t136365, t136367, t136369, t136403, t136433, t136434, t136457, t136458, t136468)
}
