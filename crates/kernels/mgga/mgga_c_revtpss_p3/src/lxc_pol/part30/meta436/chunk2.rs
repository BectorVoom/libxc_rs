//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1669/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1669<F: Float>(t1150: F, t16942: F, t1131: F, t1168: F, t5143: F, t1745: F, t3471: F, t12423: F, t16649: F, t16651: F, t16654: F, t16657: F, t16660: F, t16664: F, t16667: F, t16671: F, t16690: F, t3452: F, t5147: F) -> (F, F) {
    let t16943 = t16942 * t1150;
    let t16945 = F::new(1.0) * t1131 * t16943;
    let t16948 = t5143 * t1168;
    let t16951 = t1745 * t3471;
    let t16954 = t16649 - t16651 + t16654 + t16657 + t16660 - t16664 - t16667 - t16671 - t16690 + F::cast_from(0.64327917994770140268e2_f64) * t12423 * t5147 - F::new(4.0) * t3452 * t16948 - F::new(2.0) * t3452 * t16951;
    (t16945, t16954)
}
