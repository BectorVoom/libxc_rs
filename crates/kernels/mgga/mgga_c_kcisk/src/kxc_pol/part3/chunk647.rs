//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 647/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk647<F: Float>(t1785: F, t5030: F, t5038: F, t7261: F, t1636: F, t5015: F, t10593: F, t7242: F, t1764: F, t3934: F, t654: F, t4989: F, t5002: F, t164: F, t1786: F, t1773: F) -> (F, F, F, F, F, F) {
    let t10843 = t5030 * t1785;
    let t10844 = t10843 * t5038;
    let t10845 = t7261 * t10844;
    let t10848 = t1636 * t5038;
    let t10849 = t5015 * t10848;
    let t10852 = t7242 * t10593;
    let t10856 = t1764 * t654 * t3934;
    let t10863 = t4989 * t5002;
    let t10865 = t164 * t1786;
    let t10866 = t1773 * t10865;
    (t10845, t10849, t10852, t10856, t10863, t10866)
}
