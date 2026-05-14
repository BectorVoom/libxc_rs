//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1202/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1202<F: Float>(t34676: F, t564: F, t2053: F, t2359: F, t2776: F, t33986: F, t11200: F, t1782: F, t4826: F, t5030: F, t2642: F, t5439: F, t3179: F, t3185: F, t1009: F, t15451: F) -> (F, F, F, F, F, F, F, F) {
    let t34677 = t564 * t34676;
    let t34679 = t2359 * t2053;
    let t34680 = t2776 * t34679;
    let t35049 = 2.0 * t33986;
    let t36247 = t1782 * t11200;
    let t36267 = t5030 * t4826;
    let t36707 = t5439 * t2642;
    let t37229 = t3179 * t3185;
    let t37234 = t15451 * t1009;
    (t34677, t34680, t35049, t36247, t36267, t36707, t37229, t37234)
}
