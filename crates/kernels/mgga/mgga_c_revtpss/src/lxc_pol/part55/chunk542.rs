//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 542/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk542<F: Float>(t1150: F, t5104: F, t1131: F, t1732: F, t3435: F, t1149: F, t3433: F, t3358: F, t3439: F, t5044: F, t5049: F, t5054: F, t5058: F, t1160: F, t1737: F, t1168: F, t1745: F) -> (F, F, F, F, F) {
    let t5105 = t5104 * t1150;
    let t5107 = 1.0 * t1131 * t5105;
    let t5108 = t1732 * t3435;
    let t5109 = t5108 * t1149;
    let t5111 = 0.16081979498692535067e2 * t3433 * t5109;
    let t5117 = t3439 - 0.57077777777777777777e-2 * t3358 - 0.57077777777777777777e-2 * t5044 - 0.11415555555555555555e-1 * t5049 + 0.34246666666666666666e-1 * t5054 + 0.17123333333333333333e-1 * t5058;
    let t5120 = t1737 * t1160;
    let t5125 = t1745 * t1168;
    (t5107, t5111, t5117, t5120, t5125)
}
