//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 890/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk890<F: Float>(t28855: F, t4629: F, t11393: F, t28591: F, t706: F, t11402: F, t22417: F, t2487: F, t1887: F, t28571: F, t1882: F, t28393: F) -> (F, F, F, F, F, F, F) {
    let t28856 = t4629 * t28855;
    let t28859 = t11393 * t28591;
    let t28860 = t706 * t28859;
    let t28865 = t11402 * t22417 * t2487;
    let t28868 = t1887 * t28571;
    let t28869 = t706 * t28868;
    let t28873 = t1882 * t28393;
    (t28856, t28859, t28860, t28865, t28868, t28869, t28873)
}
