//! LDA_C_PK09 kxc pol — kxc_pol part 2 (v2rho2_1) CSE chunk 608/979 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pk09_kxc_pol_part2_v2rho2_1_chunk608<F: Float>(t1746: F, t6253: F, t1680: F, t520: F, t1743: F, t305: F, t1468: F, t534: F, t1782: F, t1792: F, t1837: F, t93: F, t1835: F, t1841: F, t1729: F, t1240: F, t902: F) -> (F, F, F, F, F, F, F, F) {
    let t6254 = t1746 * t6253;
    let t6256 = t1680 * t1680;
    let t6258 = 1.0 / t6256 / t520;
    let t6260 = t1743 * t1743;
    let t6261 = 1.0 / t6260;
    let t6262 = t6261 * t305;
    let t6266 = t534 * t1468;
    let t6267 = t6266 * t1782;
    let t6268 = t1837 * t1792;
    let t6270 = t6267 * t93 * t6268;
    let t6272 = t1835 * t1841;
    let t6273 = t1837 * t1729;
    let t6274 = t93 * t6273;
    let t6275 = t6272 * t6274;
    let t6277 = t902 * t1240;
    (t6254, t6258, t6262, t6267, t6270, t6272, t6275, t6277)
}
