//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 585/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk585<F: Float>(t2757: F, t496: F, t221: F, t2662: F, t454: F, t1074: F, t567: F, t1073: F, t475: F, t470: F, t1080: F) -> (F, F, F, F, F, F, F) {
    let t2759 = F::new(32.0) * t2757 * t496;
    let t2762 = F::cast_from(0.14764627977777777777e-2_f64) * t221 * t2662 * t454;
    let t2766 = t567 * t1074;
    let t2770 = t1073 * t475;
    let t2771 = F::new(1.0) / t2770;
    let t2772 = t470 * t2771;
    let t2773 = t1080 * t1080;
    (t2759, t2762, t2766, t2770, t2771, t2772, t2773)
}
