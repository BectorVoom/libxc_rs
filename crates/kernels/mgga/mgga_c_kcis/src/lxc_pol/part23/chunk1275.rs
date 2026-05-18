//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1275/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1275<F: Float>(t1014: F, t28476: F, t28426: F, t7895: F, t11881: F, t8165: F, t1464: F, t27423: F, t98409: F, t1593: F, t28374: F, t3999: F, t7908: F) -> (F, F, F, F, F, F) {
    let t98822 = t1014 * t28476;
    let t98823 = F::new(0.88437037037037037034e-2) * t98822;
    let t98825 = F::new(0.46336805555555555556e-3) * t7895 * t28426;
    let t98830 = t11881 * t8165;
    let t98835 = t1464 * t98409 * t27423;
    let t98845 = t7908 * t1593 * t3999 * t28374;
    (t98822, t98823, t98825, t98830, t98835, t98845)
}
