//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta686 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2503;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2504;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta686<F: Float>(t12772: F, t12780: F, t3625: F, t13052: F, t13054: F, t3172: F, t11262: F, t3711: F, t3713: F, t12657: F, t1284: F, t3624: F, t12875: F, t12916: F, t5331: F, t12871: F, t5340: F, t1222: F, t12282: F, t17471: F, t1261: F, t12944: F, t12932: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t44729, t44748, t44751, t44769) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2503::<F>(t12772, t12780, t3625, t13052, t13054, t3172, t11262, t3711, t3713, t12657, t1284, t3624);
        let (t44773, t44776, t44786, t44789, t44792) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2504::<F>(t12875, t12916, t5331, t12871, t5340, t1222, t12282, t17471, t1261, t12944, t3172, t12932, t3711);
    (t44729, t44748, t44751, t44769, t44773, t44776, t44786, t44789, t44792)
}
