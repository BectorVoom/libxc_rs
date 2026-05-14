//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 835/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk835<F: Float>(t1553: F, t4346: F, t13399: F, t1197: F, t3722: F, t13064: F, t325: F, t12884: F, t1524: F, t4460: F, t4459: F, t512: F, t507: F, t12998: F, t12974: F, t1527: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14639 = t1553 * t4346;
    let t14665 = 0.51588271604938271604e-3 * t13399;
    let t14733 = t1197 * t3722;
    let t14736 = t325 * t13064;
    let t14743 = t325 * t12884;
    let t14752 = t1524 * t4460;
    let t14756 = 1.0 / t4459 / t512;
    let t14757 = t507 * t14756;
    let t14784 = 0.46308888888888888888e0 * t12998;
    let t14785 = 0.16068111111111111111e1 * t12974;
    let t14797 = 1.0 / t4459 / t1527;
    (t14639, t14665, t14733, t14736, t14743, t14752, t14757, t14784, t14785, t14797)
}
