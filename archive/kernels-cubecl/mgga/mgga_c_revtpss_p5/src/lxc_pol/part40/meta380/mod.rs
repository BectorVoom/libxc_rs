//! MGGA_C_REVTPSS lxc pol kernel — _part40_v4rho3tau_3 meta380 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1366;
use chunk1::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1367;
use chunk2::mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1368;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_meta380<F: Float>(t16668: F, t3385: F, t12227: F, t3520: F, t5180: F, t5206: F, t1196: F, t3495: F, t1189: F, t3543: F, t5192: F, t3516: F, t5197: F, t12500: F, t5205: F, t1733: F, t3433: F, t3302: F, t5332: F, t1214: F, t5333: F, t1716: F, t2435: F, t5048: F, t689: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t16671, t16675, t16679, t16681, t16682) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1366::<F>(t16668, t3385, t12227, t3520, t5180, t5206, t1196, t3495, t1189, t3543, t5192, t3516, t5197);
        let (t16684, t16687, t16690, t16695, t16696, t16697, t16706) = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1367::<F>(t1196, t16682, t12500, t5205, t1733, t3385, t3433, t3302, t5332, t1214, t5333, t1716, t2435);
        let t16708 = mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1368::<F>(t5048, t689);
    (t16671, t16675, t16679, t16681, t16684, t16687, t16690, t16695, t16696, t16697, t16706, t16708)
}
