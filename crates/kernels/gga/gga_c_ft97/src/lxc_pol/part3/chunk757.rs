//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 757/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk757<F: Float>(t12343: F, t12346: F, t12359: F, t12362: F, t12571: F, t12891: F, t12897: F, t12911: F, t12914: F, t16706: F, t9383: F, t16710: F, t16714: F, t16717: F, t16721: F, t16724: F, t16727: F, t16730: F, t16734: F, t17237: F, t17241: F, t17244: F) -> (F, F) {
    let t17459 = t12891 - t12897 - t12343 - t12346 - t12911 + 4.0 / 27.0 * t12359 - 8.0 / 81.0 * t12362 - t9383 + t12914 - 8.0 / 27.0 * t12571 - 2.0 / 27.0 * t16706;
    let t17472 = 4.0 / 9.0 * t16710 - 2.0 / 9.0 * t16714 - 2.0 / 3.0 * t16717 + 2.0 / 27.0 * t16721 + 4.0 / 9.0 * t16724 - 10.0 / 81.0 * t16727 - 8.0 / 27.0 * t16730 + 2.0 / 9.0 * t16734 - t17237 / 12.0 + t17241 / 8.0 - t17244 / 6.0;
    (t17459, t17472)
}
