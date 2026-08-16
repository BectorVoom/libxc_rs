//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2825/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2825<F: Float>(t225: F, t42066: F, t41306: F, t3057: F, t3259: F, t367: F, t371: F, t373: F, t9291: F, t3197: F, t3201: F, t3231: F) -> (F, F, F, F, F, F) {
    let t42067 = t225 * t42066;
    let t42078 = F::cast_from(0.15365432098765432099e0_f64) * t41306;
    let t42107 = t3057 * t3259;
    let t42121 = F::cast_from(0.14820648238345094262e-3_f64) * t367 * t371 * t9291 * t373;
    let t42124 = t3197 * t3201;
    let t42141 = t3231 * t3201;
    (t42067, t42078, t42107, t42121, t42124, t42141)
}
