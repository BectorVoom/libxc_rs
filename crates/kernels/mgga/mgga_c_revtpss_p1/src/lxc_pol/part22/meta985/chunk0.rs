//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3335/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3335<F: Float>(t141: F, t63253: F, t930: F, t18281: F, t2852: F, t606: F, t2908: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t51921: F, t51923: F, t63238: F, t63240: F, t63242: F, t63246: F, t63250: F) -> (F, F, F, F) {
    let t63255 = t141 * t930 * t63253;
    let t63258 = t2852 * t18281 * t606;
    let t63260 = t141 * t2908 * t63258;
    let t63262 = -F::cast_from(0.43816888888888888888e0_f64) * t51909 + F::cast_from(0.73028148148148148146e-1_f64) * t51911 + F::cast_from(0.73028148148148148147e0_f64) * t51913 - F::cast_from(0.12171358024691358024e0_f64) * t51915 - F::cast_from(0.21908444444444444444e0_f64) * t51917 + F::cast_from(0.36514074074074074073e-1_f64) * t51921 + F::cast_from(0.48685432098765432097e-1_f64) * t51923 - F::cast_from(0.49293999999999999999e0_f64) * t63238 + F::cast_from(0.65725333333333333332e0_f64) * t63240 - F::cast_from(0.43816888888888888888e0_f64) * t63242 - F::cast_from(0.49293999999999999999e0_f64) * t63246 + F::cast_from(0.32862666666666666666e0_f64) * t63250 + F::cast_from(0.32862666666666666666e0_f64) * t63255 - F::cast_from(0.54771111111111111112e-1_f64) * t63260;
    (t63255, t63258, t63260, t63262)
}
