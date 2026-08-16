//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1214/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1214<F: Float>(t116: F, t34418: F, t13426: F, t8749: F, t125211: F, t125213: F, t125215: F, t125217: F, t125223: F, t1502: F, t28050: F, t32791: F, t33578: F, t33580: F, t33583: F, t4246: F, t671: F, t7586: F, t8756: F) -> (F, F) {
    let t129270 = t34418 * t116;
    let t129273 = t13426 * t8749;
    let t129275 = -F::cast_from(2.0_f64) * t129270 * t671 - t1502 * t32791 - F::cast_from(2.0_f64) * t28050 * t7586 - t4246 * t8756 - t125211 - t125213 - t125215 - t125217 + t125223 - F::cast_from(2.0_f64) * t129273 - t33578 - t33580 - t33583;
    (t129270, t129275)
}
