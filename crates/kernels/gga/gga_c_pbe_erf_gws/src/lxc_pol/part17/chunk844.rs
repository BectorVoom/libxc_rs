//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 844/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk844<F: Float>(t185: F, t7121: F, t1627: F, t2667: F, t2674: F, t2680: F, t2789: F, t586: F, t1824: F, t1829: F, t2615: F, t7083: F, t7084: F, t7086: F, t7091: F, t7096: F, t7100: F, t7101: F, t7105: F, t7109: F, t7113: F, t7120: F) -> (F, F, F, F, F, F, F, F) {
    let t7122 = t185 * t7121;
    let t7123 = F::cast_from(4.0_f64) / F::cast_from(135.0_f64) * t7122;
    let t7125 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t1627 * t2667;
    let t7127 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t1627 * t2674;
    let t7129 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t1627 * t2680;
    let t7130 = t2789 * t586;
    let t7132 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t7130 * t1824;
    let t7134 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2615 * t1829;
    let t7135 = -t7083 - t7084 - t7086 - t7091 - t7096 + t7100 + t7101 + t7105 - t7109 - t7113 + t7120 + t7123 - t7125 - t7127 + t7129 + t7132 - t7134;
    (t7123, t7125, t7127, t7129, t7130, t7132, t7134, t7135)
}
