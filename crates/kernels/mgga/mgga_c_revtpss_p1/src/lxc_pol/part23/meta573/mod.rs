//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta573 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2170;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2171;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2172;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta573<F: Float>(t14336: F, t14339: F, t1544: F, t18860: F, t5966: F, t45: F, t190: F, t22688: F, t10439: F, t4546: F, t18540: F, t18545: F, t18547: F, t14363: F, t22671: F, t4328: F, t5825: F, t633: F, t766: F, zeta_threshold: F, t57: F, t4335: F, t637: F, t770: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t23106, t23110, t23111, t23114) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2170::<F>(t14336, t14339, t1544, t18860, t5966);
        let (t23121, t23123, t23124, t23127, t23128, t23129, t23130, t23138) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2171::<F>(t45, t190, t22688, t10439, t4546, t5966, t18540, t18545, t18547, t14363, t22671, t4328, t5825, t633, t766, zeta_threshold);
        let t23148 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2172::<F>(t57, t22671, t22688, t4335, t5825, t637, t770, t23138, zeta_threshold);
    (t23106, t23110, t23111, t23114, t23121, t23123, t23124, t23127, t23128, t23129, t23130, t23148)
}
