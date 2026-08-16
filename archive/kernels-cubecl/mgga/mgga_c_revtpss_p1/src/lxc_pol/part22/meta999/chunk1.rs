//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3392/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3392<F: Float>(t19150: F, t3022: F, t51909: F, t51911: F, t51913: F, t51915: F, t51917: F, t51921: F, t51923: F, t63238: F, t63240: F, t63242: F, t63246: F, t63250: F, t63255: F, t63260: F) -> (F, F) {
    let t63685 = F::cast_from(0.46785788981077169656e1_f64) * t3022 * t19150;
    let t63700 = -F::cast_from(0.44152e0_f64) * t51909 + F::cast_from(0.73586666666666666666e-1_f64) * t51911 + F::cast_from(0.73586666666666666667e0_f64) * t51913 - F::cast_from(0.12264444444444444444e0_f64) * t51915 - F::cast_from(0.22076e0_f64) * t51917 + F::cast_from(0.36793333333333333333e-1_f64) * t51921 + F::cast_from(0.49057777777777777777e-1_f64) * t51923 - F::cast_from(0.49671e0_f64) * t63238 + F::cast_from(0.66228e0_f64) * t63240 - F::cast_from(0.44152e0_f64) * t63242 - F::cast_from(0.49671e0_f64) * t63246 + F::cast_from(0.33114e0_f64) * t63250 + F::cast_from(0.33114e0_f64) * t63255 - F::cast_from(0.5519e-1_f64) * t63260;
    (t63685, t63700)
}
