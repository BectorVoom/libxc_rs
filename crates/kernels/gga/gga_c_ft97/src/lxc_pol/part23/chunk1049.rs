//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1049/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1049<F: Float>(t2881: F, t31747: F, t1901: F, t29111: F, t31691: F, t31695: F, t31699: F, t31703: F, t31706: F, t31710: F, t31714: F, t31717: F, t31721: F, t31724: F, t31729: F, t31732: F, t31736: F, t31740: F, t31744: F, t446: F) -> (F, F) {
    let t31748 = t2881 * t31747;
    let t31751 = -2.0 / 3.0 * t446 * t31691 + 2.0 / 3.0 * t446 * t31695 - 2.0 / 9.0 * t1901 * t31699 - 4.0 / 9.0 * t1901 * t31703 - t446 * t31706 / 3.0 + t1901 * t31710 / 9.0 + 2.0 / 9.0 * t1901 * t31714 + 2.0 / 9.0 * t1901 * t31717 - 2.0 / 9.0 * t1901 * t31721 + 2.0 / 9.0 * t1901 * t31724 - 2.0 / 27.0 * t29111 - 2.0 / 3.0 * t446 * t31729 + 2.0 / 9.0 * t1901 * t31732 + 2.0 / 9.0 * t1901 * t31736 + t1901 * t31740 / 9.0 + 2.0 / 27.0 * t1901 * t31744 - 2.0 / 9.0 * t1901 * t31748;
    (t31748, t31751)
}
