//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 882/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk882<F: Float>(t1556: F, t4495: F, t1553: F, t4346: F, t13399: F, t1203: F, t3688: F, t1197: F, t3722: F, t13064: F, t325: F, t12884: F, t1528: F, t4428: F, t1524: F, t4460: F) -> (F, F, F, F, F, F, F, F, F) {
    let t14636 = t4495 * t1556;
    let t14639 = t1553 * t4346;
    let t14665 = 0.51588271604938271604e-3 * t13399;
    let t14728 = t3688 * t1203;
    let t14733 = t1197 * t3722;
    let t14736 = t325 * t13064;
    let t14743 = t325 * t12884;
    let t14747 = t4428 * t1528;
    let t14752 = t1524 * t4460;
    (t14636, t14639, t14665, t14728, t14733, t14736, t14743, t14747, t14752)
}
