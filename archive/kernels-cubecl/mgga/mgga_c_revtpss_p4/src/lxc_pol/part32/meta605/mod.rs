//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta605 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1943;
use chunk1::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1944;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta605<F: Float>(t105944: F, t1955: F, t5978: F, t886: F, t1558: F, t231: F, t4533: F, t6048: F, t836: F, t6071: F, t105945: F, t7063: F, t18657: F, t1579: F, t4423: F, t25207: F, t77441: F, t1544: F, t580: F, t98646: F, t18435: F, t27159: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t106275, t106290, t106302, t106360, t106365, t106387) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1943::<F>(t105944, t1955, t5978, t886, t1558, t231, t4533, t6048, t836, t6071, t105945, t7063);
        let (t106404, t106410, t106490, t106494, t106498) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1944::<F>(t18657, t1955, t1579, t231, t4423, t25207, t77441, t1544, t580, t98646, t18435, t27159);
    (t106275, t106290, t106302, t106360, t106365, t106387, t106404, t106410, t106490, t106494, t106498)
}
