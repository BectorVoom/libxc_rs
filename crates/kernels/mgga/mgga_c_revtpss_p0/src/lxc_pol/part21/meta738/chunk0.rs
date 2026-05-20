//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2591/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2591<F: Float>(t2435: F, t9635: F, t9590: F, t9593: F, t10179: F, t1450: F, t4146: F, t1455: F, t5808: F, t46279: F, t46281: F, t46286: F) -> (F, F, F, F, F, F, F, F) {
    let t47620 = t2435 * t9635;
    let t47638 = t9590 * t9593;
    let t47651 = t10179 * t1450;
    let t47671 = t4146 * t4146;
    let t47672 = F::new(1.0) / t47671;
    let t47730 = t1455 * t5808;
    let t47753 = F::new(36.0) * t46279;
    let t47754 = F::new(180.0) * t46281;
    let t47758 = F::cast_from(0.17544670867903938621e1_f64) * t46286;
    (t47620, t47638, t47651, t47672, t47730, t47753, t47754, t47758)
}
