//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1869/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1869(t20811: f64, t20812: f64, t20821: f64, t20832: f64, t225: f64, t20756: f64, t9946: f64, t4226: f64, t5544: f64, t20800: f64, t824: f64, t1504: f64, t1506: f64, t228: f64, t230: f64, t4225: f64, t5601: f64, t5605: f64, t5608: f64) -> (f64, f64, f64, f64, f64) {
    let t20835 = (t20811 + t20812 + t20821 + t20832) * t225;
    let t20843 = t9946 * t20756;
    let t20846 = t4226 * t5544;
    let t20849 = t824 * t20800;
    let t20852 = -36.0_f64 * t1504 * t5605 + 9.0_f64 * t1504 * t5608 + 9.0_f64 * t1506 * t5601 - t20835 * t230 + 60.0_f64 * t20843 * t228 - 36.0_f64 * t20846 * t4225 + 3.0_f64 * t20849 * t228;
    (t20835, t20843, t20846, t20849, t20852)
}
