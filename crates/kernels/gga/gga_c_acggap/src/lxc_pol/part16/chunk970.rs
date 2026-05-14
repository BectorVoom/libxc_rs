//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 970/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk970<F: Float>(t1181: F, t1844: F, t2068: F, t360: F, t604: F, t30268: F, t9589: F, t372: F, t1165: F, t7351: F, t570: F, t6175: F, t5636: F, t1745: F, t2009: F, t1988: F, t9549: F) -> (F, F, F, F, F, F, F, F) {
    let t39160 = t2068 * t1181 * t604 * t1844 * t360;
    let t39162 = t30268 * t9589;
    let t39164 = t1844 * t372;
    let t39167 = t2068 * t1165 * t7351 * t39164;
    let t39169 = t570 * t6175;
    let t39171 = t570 * t5636;
    let t39173 = t2009 * t1745;
    let t39176 = t1988 * t9549;
    (t39160, t39162, t39164, t39167, t39169, t39171, t39173, t39176)
}
