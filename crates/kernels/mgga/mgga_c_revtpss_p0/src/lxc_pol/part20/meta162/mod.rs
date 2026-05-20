//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta162 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk872;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk873;
use chunk2::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk874;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta162<F: Float>(t198: F, t532: F, t539: F, t73: F, t241: F, t4000: F, t820: F, t550: F, t72: F, t245: F, t1398: F, t4003: F, t225: F, t3999: F, t213: F, t4086: F, t640: F, t76: F, t159: F, t793: F, t1448: F, t4147: F, t587: F, t65: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t5541, t5650, t5671, t5673) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk872::<F>(t198, t532, t539, t73, t241, t4000, t820, t550, t72, t245);
        let (t5675, t5744) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk873::<F>(t1398, t4003, t225, t3999);
        let (t5745, t5755, t6977, t7021, t7315, t8779) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk874::<F>(t213, t5744, t4086, t640, t76, t159, t793, t1448, t4147, t587, t65);
    (t5541, t5650, t5671, t5673, t5675, t5744, t5745, t5755, t6977, t7021, t7315, t8779)
}
