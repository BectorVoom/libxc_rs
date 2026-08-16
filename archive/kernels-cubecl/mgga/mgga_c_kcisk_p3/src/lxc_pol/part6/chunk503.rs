//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 503/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk503<F: Float>(t41: F, t4594: F, t4597: F, t702: F, t1849: F, t5060: F, t732: F, t1934: F, t718: F, t642: F, t5061: F, t740: F, sigma2: F) -> (F, F, F, F, F, F, F, F) {
    let t5248 = t41 * t4594;
    let t5249 = t702 * t4597;
    let t5259 = t702 * t1849;
    let t5283 = t732 * t5060;
    let t5284 = t5283 * sigma2;
    let t5289 = t1934 * t718;
    let t5290 = t41 * t642;
    let t5315 = t5061 * t740;
    (t5248, t5249, t5259, t5283, t5284, t5289, t5290, t5315)
}
