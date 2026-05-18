//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 890/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk890<F: Float>(t1045: F, t13677: F, t331: F, t4845: F, t1027: F, t4849: F, t10114: F, t4840: F, t4852: F, t1670: F, t2944: F, t4625: F, t934: F) -> (F, F, F, F, F, F, F) {
    let t13678 = t13677 * t1045;
    let t13682 = F::new(0.93706135855523581992e-2) * t331 * t4845;
    let t13684 = F::new(0.28111840756657074598e-1) * t1027 * t4849;
    let t13686 = t10114 * t4840;
    let t13689 = F::new(0.93706135855523581992e-2) * t1027 * t4852;
    let t13691 = t1670 * t2944;
    let t13696 = t4625 * t934;
    (t13678, t13682, t13684, t13686, t13689, t13691, t13696)
}
