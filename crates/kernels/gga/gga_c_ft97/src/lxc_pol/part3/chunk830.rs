//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 830/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk830<F: Float>(t18396: F, t18429: F, t18454: F, t18511: F, t18599: F, t18639: F, t18706: F, t18754: F, t1137: F, t1173: F, t17713: F, t17715: F, t17718: F, t18178: F, t247: F, t263: F, t3683: F, t3827: F, t4003: F, t4915: F, t5059: F, t5179: F, t719: F, t771: F) -> (F,) {
    let t18757 = t18396 + t18429 + t18454 + t18511 + t18599 + t18639 + t18706 + t18754;
    let t18759 = -2.0 * t1137 * t4003 - 2.0 * t1173 * t3683 - 2.0 * t1173 * t3827 - t17713 * t263 - t17715 * t263 - t17718 * t263 - t18178 * t263 - t18757 * t247 - t4915 * t771 - t5059 * t771 - t5179 * t719;
    (t18759,)
}
