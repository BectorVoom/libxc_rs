//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta482 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1939;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1940;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1941;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta482<F: Float>(t30: F, t265: F, t393: F, t18884: F, t19141: F, t20234: F, t1106: F, t1468: F, t1469: F, t1704: F, t18280: F, t18281: F, t18892: F, t395: F, t4186: F, t45: F, t4560: F, t5028: F, t5824: F, t5825: F, t605: F, t606: F, t6084: F, t6405: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t3531: F, t6556: F, t6552: F, t3362: F, t3417: F, t141: F, t1121: F) -> (F, F, F, F, F, F, F, F, F, F) {
        let (t20236, t20248) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1939::<F>(t30, t265, t393, t18884, t19141, t20234, t1106, t1468, t1469, t1704, t18280, t18281, t18892, t395, t4186, t45, t4560, t5028, t5824, t5825, t605, t606, t6084, t6405, t895, dens_threshold, rho0, zeta_threshold);
        let t20256 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1940::<F>(t18280);
        let (t20261, t20263, t20265, t20266, t20267, t20268, t20272) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1941::<F>(t3531, t6556, t6552, t3362, t5825, t606, t3417, t141, t1121, t18281);
    (t20236, t20248, t20256, t20261, t20263, t20265, t20266, t20267, t20268, t20272)
}
