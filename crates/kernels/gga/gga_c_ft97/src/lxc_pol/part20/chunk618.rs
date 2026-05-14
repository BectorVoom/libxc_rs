//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 618/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk618<F: Float>(t13706: F, t9744: F, t446: F, t1131: F, t2373: F, t7514: F, t193: F, t89: F, t1087: F, t9733: F, t13700: F, t13704: F, t9520: F, t9701: F, t9723: F, t9727: F, t9730: F, t9735: F) -> (F, F, F, F) {
    let t13707 = t9744 * t13706;
    let t13708 = t446 * t13707;
    let t13717 = t7514 * t1131 * t2373;
    let t13719 = t89 * t193 * t13717;
    let t13722 = t89 * t9733 * t1087;
    let t13723 = 4.0 / 81.0 * t13722;
    let t13724 = t13700 / 6.0 - 4.0 / 9.0 * t13704 + 4.0 / 27.0 * t13708 + t9723 / 27.0 + 2.0 / 81.0 * t9727 - 8.0 / 81.0 * t9735 - 8.0 / 27.0 * t9701 - 2.0 / 9.0 * t9730 + t9520 / 9.0 - 2.0 * t13719 - t13723;
    (t13708, t13719, t13722, t13724)
}
