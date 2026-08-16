//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2507/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2507<F: Float>(t14724: F, t9775: F, t10722: F, t4435: F, t10716: F, t14757: F, t10868: F, t2482: F, t814: F, t10845: F, t14732: F, t4423: F, t853: F) -> (F, F, F, F, F, F) {
    let t50504 = t9775 * t14724;
    let t50505 = F::cast_from(0.22866142996303859718e-3_f64) * t50504;
    let t50524 = t10722 * t4435;
    let t50531 = t10716 * t14757;
    let t50532 = F::cast_from(0.8131200449485652516e-2_f64) * t50531;
    let t50570 = t2482 * t10868 * t814;
    let t50581 = t10845 * t14732;
    let t50582 = F::cast_from(0.40656002247428262579e-3_f64) * t50581;
    let t50583 = t853 * t4423;
    (t50505, t50524, t50532, t50570, t50582, t50583)
}
