//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 634/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk634<F: Float>(t10487: F, t167: F, t11458: F, t1049: F, t695: F, t1849: F, t642: F, t1906: F, t724: F, t11225: F, t732: F, t640: F, t719: F, t702: F, t140: F, t446: F, t728: F, sigma2: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11630 = t167 * t10487;
    let t11633 = 0.71734315950379065738e-1 * t11458;
    let t11634 = t1049 * t695;
    let t11635 = 0.62154466893555682512e-3 * t11634;
    let t11682 = t642 * t1849;
    let t11699 = t1906 * t1906;
    let t11700 = 1.0 / t11699;
    let t11701 = t724 * t11700;
    let t11774 = t732 * t11225;
    let t11775 = t11774 * sigma2;
    let t11807 = 1.0 / t719 / t640;
    let t11832 = t702 * t10487;
    let t11885 = 0.11791604938271604938e-1 * t140 * t446 * t728;
    (t11630, t11633, t11635, t11682, t11701, t11775, t11807, t11832, t11885)
}
