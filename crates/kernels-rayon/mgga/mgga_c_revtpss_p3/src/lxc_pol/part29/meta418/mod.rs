//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta418 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1540;
use chunk1::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1541;
use chunk2::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1542;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta418(t16668: f64, t3385: f64, t12227: f64, t3520: f64, t5180: f64, t5206: f64, t1196: f64, t3495: f64, t1189: f64, t3543: f64, t5192: f64, t3516: f64, t5197: f64, t12500: f64, t5205: f64, t1733: f64, t3433: f64, t3302: f64, t5332: f64, t1214: f64, t5333: f64, t1716: f64, t2435: f64, t5048: f64, t689: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t16671, t16675, t16679, t16681, t16682) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1540(t16668, t3385, t12227, t3520, t5180, t5206, t1196, t3495, t1189, t3543, t5192, t3516, t5197);
        let (t16684, t16687, t16690, t16695, t16696, t16697, t16706) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1541(t1196, t16682, t12500, t5205, t1733, t3385, t3433, t3302, t5332, t1214, t5333, t1716, t2435);
        let t16708 = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1542(t5048, t689);
    (t16671, t16675, t16679, t16681, t16684, t16687, t16690, t16695, t16696, t16697, t16706, t16708)
}
