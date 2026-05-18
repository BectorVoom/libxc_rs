//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 761/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk761<F: Float>(t11726: F, t740: F, t748: F, t1929: F, t5060: F, t5286: F, t11450: F, t747: F, t746: F, t1948: F, t10479: F, t7303: F, sigma2: F) -> (F, F, F, F) {
    let t11727 = t11726 * t740;
    let t11728 = t11727 * t748;
    let t11730 = t1929 * t5060;
    let t11731 = t11730 * sigma2;
    let t11732 = t11731 * t5286;
    let t11734 = t747 * t11450;
    let t11735 = t746 * t11734;
    let t11736 = t1948 * t11735;
    let t11738 = t7303 * t10479;
    (t11728, t11732, t11736, t11738)
}
