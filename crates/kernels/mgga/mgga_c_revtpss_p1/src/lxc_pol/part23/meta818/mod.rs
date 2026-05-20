//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta818 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2665;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2666;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta818<F: Float>(t19676: F, t3127: F, t3172: F, t16158: F, t4834: F, t19791: F, t19781: F, t3091: F, t43131: F, t19939: F, t11262: F, t3161: F, t6311: F, t11274: F, t20029: F, t11656: F, t19920: F, t6262: F, t15817: F, t4820: F, t15775: F, t1032: F, t1040: F, t19856: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t65527, t65538, t65553, t65567, t65570, t65581) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2665::<F>(t19676, t3127, t3172, t16158, t4834, t19791, t19781, t3091, t43131, t19939, t11262, t3161, t6311);
        let (t65585, t65589, t65596, t65598, t65610, t65613) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2666::<F>(t11274, t20029, t11656, t19920, t11262, t3127, t6262, t15817, t4820, t15775, t4834, t1032, t1040, t19856);
    (t65527, t65538, t65553, t65567, t65570, t65581, t65585, t65589, t65596, t65598, t65610, t65613)
}
