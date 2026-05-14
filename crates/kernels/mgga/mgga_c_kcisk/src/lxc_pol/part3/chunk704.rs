//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 704/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk704<F: Float>(t10777: F, t196: F, t10585: F, t7370: F, t1849: F, t1860: F, t1919: F, t3290: F, t10441: F, t5249: F, t4265: F, t5251: F, t10487: F, t702: F, t5248: F, t3293: F, t5254: F) -> (F, F, F, F, F, F, F) {
    let t11815 = t10777 * t196;
    let t11818 = t7370 * t10585;
    let t11821 = t1860 * t1849;
    let t11823 = t1919 * t11821 * t3290;
    let t11827 = t1919 * t5249 * t10441;
    let t11830 = t4265 * t5251;
    let t11832 = t702 * t10487;
    let t11834 = t5248 * t11832 * t10441;
    let t11838 = t1919 * t5254 * t3293;
    (t11815, t11818, t11823, t11827, t11830, t11834, t11838)
}
