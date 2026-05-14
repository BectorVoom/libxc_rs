//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1039/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1039<F: Float>(t127354: F, t28196: F, t28198: F, t28056: F, t8634: F, t32129: F, t7898: F, t2007: F, t28042: F, t651: F, t13426: F, t8461: F, t127335: F, t127336: F, t127340: F, t127341: F, t127346: F, t127349: F, t1932: F, t27830: F, t28053: F, t32107: F, t32109: F, t32112: F, t6983: F, t6985: F, t7883: F, t8463: F) -> (F,) {
    let t127357 = 6.0 * t28196 * t127354 * t28198;
    let t127359 = 4.0 * t8634 * t28056;
    let t127361 = 2.0 * t7898 * t32129;
    let t127363 = t651 * t2007 * t28042;
    let t127365 = t13426 * t8461;
    let t127366 = 2.0 * t127365;
    let t127367 = -2.0 * t1932 * t27830 - 4.0 * t28053 * t6985 - 2.0 * t6983 * t7883 + t127335 - 6.0 * t127336 - t127340 + 6.0 * t127341 - t127346 + t127349 - t127357 - t127359 + t127361 - 4.0 * t127363 - t127366 - t32107 - t32109 - t32112 - t8463;
    (t127367,)
}
