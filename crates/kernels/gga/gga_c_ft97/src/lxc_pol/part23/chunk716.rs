//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 716/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk716<F: Float>(t13739: F, t13747: F, t13754: F, t13781: F, t13795: F, t13810: F, t18142: F, t18145: F, t18148: F, t18363: F, t18367: F, t13812: F, t18153: F, t18157: F, t18162: F, t18165: F, t18168: F, t18171: F, t18174: F, t18372: F, t18375: F, t9972: F) -> (F, F) {
    let t18575 = -8.0 / 27.0 * t13739 - t13747 + t13754 - t18142 / 3.0 - 2.0 / 9.0 * t18145 + t18148 / 9.0 - t13781 + t13795 - t13810 + t18363 / 6.0 - t18367 / 12.0;
    let t18585 = t18372 / 8.0 - t18375 / 6.0 + 2.0 / 3.0 * t18153 - t18157 / 9.0 - 2.0 * t18162 + 4.0 / 3.0 * t18165 + t18168 / 27.0 - 2.0 / 27.0 * t18171 + 2.0 / 81.0 * t18174 - t9972 - t13812;
    (t18575, t18585)
}
