//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 958/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk958<F: Float>(t10697: F, t4299: F, t7672: F, t112390: F, t6374: F, t34281: F, t6963: F, t1466: F, t36016: F, t681: F, t1091: F, t111711: F, t142501: F, t142503: F, t142595: F, t142597: F, t142950: F, t193: F, t25459: F, t2665: F, t29416: F, t34322: F, t35810: F, t6216: F, t6222: F, t7614: F) -> (F, F, F) {
    let t152631 = t10697 * t7672 * t4299;
    let t152633 = t112390 * t6374;
    let t152635 = t6963 * t34281;
    let t152638 = t1466 * t681 * t36016;
    let t152644 = -t6216 * t2665 * t142950 * t1091 / 18.0 + t29416 * t7614 / 6.0 - 2.0 / 3.0 * t1466 * t193 * t6222 * t111711 - t142501 / 18.0 - t142503 / 18.0 + t6963 * t34322 / 6.0 - 12.0 * t152631 + 8.0 * t152633 + t152635 / 9.0 - t152638 / 18.0 - t25459 * t35810 / 18.0 + t142595 / 54.0 + t142597 / 27.0;
    (t152631, t152633, t152644)
}
