//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1148/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1148<F: Float>(t28455: F, t8392: F, t2492: F, t6907: F, t28184: F, t1882: F, t28434: F, t1443: F, t2372: F, t108097: F, t11593: F, t13839: F, t13858: F, t13887: F, t14127: F, t1901: F, t24433: F, t24659: F, t24747: F, t2599: F, t2602: F, t28204: F, t3746: F, t3859: F, t3881: F, t42339: F, t42575: F, t53662: F, t6135: F, t68135: F, t97561: F, t97584: F, t97701: F, t97928: F) -> (F,) {
    let t110364 = 2.0 / 27.0 * t8392 * t28455;
    let t110369 = t2492 * t6907;
    let t110380 = 4.0 / 27.0 * t8392 * t28184;
    let t110400 = 4.0 / 9.0 * t1882 * t28434;
    let t110401 = t2372 * t1443;
    let t110405 = 2.0 / 27.0 * t97561 - t110364 - 4.0 / 9.0 * t11593 * t2599 * t24747 * t3746 + 2.0 / 9.0 * t1901 * t110369 * t2602 + 2.0 / 3.0 * t1901 * t53662 * t108097 - 4.0 / 9.0 * t1901 * t68135 * t24433 + t110380 + 2.0 / 9.0 * t1901 * t42339 * t6135 * t13858 - 2.0 / 9.0 * t1901 * t42575 * t28204 + t1901 * t13839 * t24659 / 9.0 + 2.0 / 9.0 * t1901 * t97701 * t3881 - 2.0 / 27.0 * t97584 - 4.0 / 3.0 * t1901 * t14127 * t97928 * t3859 - t110400 - 4.0 / 3.0 * t1901 * t110401 * t13887;
    (t110405,)
}
