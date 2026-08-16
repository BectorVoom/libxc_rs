//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 33 (v4rho3sigma_9) CSE chunk 1277/1415 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part33_v4rho3sigma_9_chunk1277<F: Float>(t28192: F, t80727: F, t22892: F, t7691: F, t90544: F, t28200: F, t6883: F, t225: F, t28053: F, t28237: F, t532: F, t2752: F, t28447: F) -> (F, F, F, F, F, F) {
    let t97664 = t80727 * t28192;
    let t97732 = t22892 * t90544 * t7691;
    let t97750 = t6883 * t28200;
    let t97756 = t28053 * t225;
    let t97817 = t532 * t28237;
    let t98054 = t28447 * t2752;
    (t97664, t97732, t97750, t97756, t97817, t98054)
}
