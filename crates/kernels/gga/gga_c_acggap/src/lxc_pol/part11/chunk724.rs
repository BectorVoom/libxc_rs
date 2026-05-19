//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 724/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk724<F: Float>(t1994: F, t7637: F, t601: F, t7630: F, t1101: F, t599: F, t1181: F, t7493: F, t168: F, t7559: F) -> (F, F, F, F, F, F, F) {
    let t7638 = t7637 * t1994;
    let t7639 = F::cast_from(0.13976929906490734252e-2_f64) * t7638;
    let t7640 = t7630 * t601;
    let t7641 = F::cast_from(0.12862205435420921092e-2_f64) * t7640;
    let t7642 = t599 * t1101;
    let t7643 = t1181 * t7642;
    let t7644 = t7493 * t7643;
    let t7645 = F::cast_from(0.10718504529517434243e-2_f64) * t7644;
    let t7646 = t7559 * t168;
    (t7639, t7641, t7642, t7643, t7644, t7645, t7646)
}
