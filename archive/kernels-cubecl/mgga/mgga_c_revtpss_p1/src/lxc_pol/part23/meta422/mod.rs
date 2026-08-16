//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta422 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1807;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1808;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta422<F: Float>(t18281: F, t190: F, t706: F, t14441: F, t10593: F, t10597: F, t189: F, t5819: F, t606: F, t14330: F, t10608: F, t4308: F, t4311: F, t10613: F, t10592: F, t10596: F, t10604: F, t10611: F, t14433: F, t14618: F, t9524: F, t9542: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t18569, t18571, t18572, t18573, t18574, t18575, t18576, t18578, t18579, t18581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1807::<F>(t18281, t190, t706, t14441, t10593, t10597, t189, t5819, t606, t14330, t10608, t4308, t4311);
        let (t18582, t18583) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1808::<F>(t10613, t10592, t10596, t10604, t10611, t14433, t14618, t18571, t18572, t18573, t18574, t18578, t18579, t18581, t9524, t9542);
    (t18569, t18571, t18572, t18573, t18574, t18575, t18576, t18578, t18579, t18581, t18582, t18583)
}
