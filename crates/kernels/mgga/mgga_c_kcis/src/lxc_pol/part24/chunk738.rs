//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 738/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk738<F: Float>(t331: F, t4837: F, t1035: F, t167: F, t4845: F, t1027: F, t4849: F, t10114: F, t4840: F, t4852: F, t1663: F, t2323: F) -> (F, F, F, F, F, F, F) {
    let t13667 = 0.93706135855523581992e-2 * t331 * t4837;
    let t13677 = t1035 * t167;
    let t13682 = 0.93706135855523581992e-2 * t331 * t4845;
    let t13684 = 0.28111840756657074598e-1 * t1027 * t4849;
    let t13686 = t10114 * t4840;
    let t13689 = 0.93706135855523581992e-2 * t1027 * t4852;
    let t13710 = t2323 * t1663;
    (t13667, t13677, t13682, t13684, t13686, t13689, t13710)
}
