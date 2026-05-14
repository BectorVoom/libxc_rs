//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1154/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1154<F: Float>(t28142: F, t8392: F, t255: F, t41848: F, t256: F, t28300: F, t28128: F, t53798: F, t53891: F, t6161: F, t10007: F, t108012: F, t11593: F, t13757: F, t13897: F, t14133: F, t14163: F, t14167: F, t14176: F, t1901: F, t2409: F, t2606: F, t27753: F, t27757: F, t27763: F, t28355: F, t28378: F, t42575: F, t52018: F, t53658: F, t53797: F, t53910: F, t6074: F, t6135: F, t97705: F, t97772: F, t97790: F) -> (F,) {
    let t110659 = 4.0 / 3.0 * t8392 * t28142;
    let t110660 = t41848 * t255;
    let t110665 = t256 * t28300;
    let t110669 = t53798 * t28128;
    let t110692 = t53891 * t6161;
    let t110700 = 16.0 / 27.0 * t97772 + 4.0 / 9.0 * t53797 * t97705 * t14167 + t110659 + 8.0 * t1901 * t110660 * t6074 * t14133 + 4.0 / 3.0 * t53797 * t110665 * t14176 + 4.0 / 9.0 * t53797 * t110669 * t14176 - 4.0 / 9.0 * t1901 * t52018 * t27753 - 2.0 / 9.0 * t1901 * t42575 * t28378 - 4.0 / 9.0 * t1901 * t53910 * t27757 + 4.0 / 27.0 * t1901 * t53658 * t27763 + 8.0 / 9.0 * t11593 * t14163 * t108012 + 4.0 / 9.0 * t11593 * t10007 * t6135 * t13897 + 4.0 / 9.0 * t53797 * t110692 * t13757 - t97790 - 2.0 / 9.0 * t1901 * t2606 * t28355 * t2409;
    (t110700,)
}
