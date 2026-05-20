//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1458;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1459;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta475<F: Float>(t11509: F, t6205: F, t2967: F, t6152: F, t3011: F, t6184: F, t2942: F, t2923: F, t6104: F, t3056: F, t6234: F, t378: F, t1063: F, t247: F, t42447: F, t6092: F, t3140: F, t6235: F, t3149: F, t11986: F, t6100: F, t11262: F, t3161: F, t6311: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t64043, t64060, t64125, t64319, t64336, t64686, t64687) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1458::<F>(t11509, t6205, t2967, t6152, t3011, t6184, t2942, t2923, t6104, t3056, t6234, t378);
        let (t65292, t65338, t65339, t65357, t65581) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1459::<F>(t1063, t247, t42447, t6092, t3140, t6235, t3149, t11986, t6100, t11262, t3161, t6311);
    (t64043, t64060, t64125, t64319, t64336, t64686, t64687, t65292, t65338, t65339, t65357, t65581)
}
